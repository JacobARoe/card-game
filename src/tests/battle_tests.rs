use crate::cli::PlayCardRequest;
use crate::components::*;
use crate::resources::{DiscardPile, GameMap, RunState};
use crate::scene_battle::{CardAnimating, process_play_card_requests};
use crate::states::{GameState, TurnState};
use bevy::prelude::*;

#[test]
fn test_cli_play_card_request() {
    let mut app = App::new();
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::time::TimePlugin);

    // Init state logic
    app.init_state::<GameState>();
    app.init_state::<TurnState>();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Battle);
    app.world_mut()
        .resource_mut::<NextState<TurnState>>()
        .set(TurnState::PlayerTurn);

    // Provide Resource Dependencies
    app.insert_resource(RunState {
        character_class: CharacterClass::Spellweaver,
    });
    app.insert_resource(GameMap::default());
    app.insert_resource(DiscardPile::default());

    // Mock standard systems & events
    app.add_event::<PlayCardRequest>();
    app.add_systems(Update, process_play_card_requests);

    // Mock Entities
    // 1. Hand Container
    let hand_container = app
        .world_mut()
        .spawn((HandContainer, NodeBundle::default()))
        .id();

    // 2. Card in Hand
    let strike_card = Card {
        name: "Strike".to_string(),
        cost: 1,
        damage: 6,
        block: 0,
        apply_poison: 0,
        apply_weak: 0,
        apply_stun: 0,
        element: SpellElement::Neutral,
        upgraded: false,
        rarity: Rarity::Common,
        is_spell_modifier: false,
        is_spell_source: false,
        combo_points_granted: 0,
        finisher_combo_cost: 0,
    };

    let card_entity = app
        .world_mut()
        .spawn((strike_card.clone(), NodeBundle::default()))
        .id();
    app.world_mut()
        .entity_mut(hand_container)
        .add_child(card_entity);

    // 3. Player entity
    app.world_mut().spawn((
        Player,
        Health {
            current: 50,
            max: 50,
        },
        Energy { current: 3, max: 3 },
        Mana { current: 3 },
        Block { value: 0 },
        StatusStore {
            poison: 0,
            weak: 0,
            burning: 0,
            frozen: 0,
            stun: 0,
            strength: 0,
        },
        RelicStore { relics: vec![] },
        ActiveSpell {
            bonus_damage: 0,
            bonus_block: 0,
            element: SpellElement::Neutral,
            essences: vec![],
            essence_history: vec![],
        },
    ));

    // 4. Enemy entity
    let enemy_entity = app
        .world_mut()
        .spawn((
            Enemy {
                kind: EnemyKind::Orc,
            },
            Health {
                current: 20,
                max: 20,
            },
            Block { value: 0 },
            StatusStore {
                poison: 0,
                weak: 0,
                burning: 0,
                frozen: 0,
                stun: 0,
                strength: 0,
            },
            GlobalTransform::default(),
        ))
        .id();

    // 5. Minimal Window
    app.world_mut().spawn(Window::default());

    app.update(); // Process spawns

    // Send Play Card Event (card 0, targeting enemy 0)
    app.world_mut()
        .resource_mut::<Events<PlayCardRequest>>()
        .send(PlayCardRequest {
            hand_index: 0,
            target_index: Some(0),
        });

    app.update(); // Process PlayCardRequest

    // Assert: Card is Animating
    let has_animating = app
        .world_mut()
        .entity(card_entity)
        .contains::<CardAnimating>();
    assert!(
        has_animating,
        "Card should have the CardAnimating component attached."
    );

    // Assert: Mana was consumed
    let mut query = app.world_mut().query::<&Mana>();
    let mana = query.single(app.world());
    assert_eq!(mana.current, 2, "Mana should be reduced by 1");
}

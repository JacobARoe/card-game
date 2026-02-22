use crate::cli::MapNodeSelectRequest;
use crate::components::NodeType;
use crate::resources::GameMap;
use crate::scene_map::process_map_node_select_requests;
use crate::states::GameState;
use bevy::prelude::*;

#[test]
fn test_cli_map_node_selection() {
    // Setup Headless Bevy App
    let mut app = App::new();

    // Setup basic Bevy State structure needed for Bevy 0.14+
    app.add_plugins(bevy::state::app::StatesPlugin);

    // Init minimal dependencies for the test
    app.init_state::<GameState>();

    // Create a mock loaded GameMap
    let mut mock_map = GameMap {
        levels: vec![
            vec![crate::resources::MapNodeData {
                node_type: NodeType::Battle,
                next_indices: vec![0],
                visible: true,
            }], // Lvl 0
        ],
        current_node: None,
        visited_path: vec![],
    };
    app.insert_resource(mock_map);

    // Register Event and Systems
    app.add_event::<MapNodeSelectRequest>();
    app.add_systems(Update, process_map_node_select_requests);

    // Initial state should be Map
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Map);
    app.update(); // Initialize systems

    // Act: Send CLI Event for Map Node 0
    app.world_mut()
        .resource_mut::<Events<MapNodeSelectRequest>>()
        .send(MapNodeSelectRequest { index: 0 });

    // Run the `Update` loop to process the event
    app.update();

    // Assert: We should have transitioned to GameState::Battle
    let next_state = app.world().resource::<NextState<GameState>>();
    match next_state {
        NextState::Pending(s) => assert_eq!(*s, GameState::Battle),
        _ => panic!("Expected Pending state!"),
    }

    // Assert: Map should have tracked visited node
    let map = app.world().resource::<GameMap>();
    assert_eq!(map.current_node, Some((0, 0)));
}

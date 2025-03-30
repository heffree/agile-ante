use crate::helpers::TestApp;

#[tokio::test]
async fn create_room_persists_to_db() {
    // Arrange
    let app = TestApp::spawn().await;

    // Act
    let res = app.create_room().await;
    println!("{:?}", res);

    // Assert
    let room_ids = app.get_rooms().await;
    assert_eq!(room_ids.len(), 1);
}

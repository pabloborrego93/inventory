use rocket::serde::json::Json;
use crate::dtos::inventory::InventoryGetDto;

#[get("/inventory")]
pub fn get_inventory() -> Json<InventoryGetDto> {
    let inventory_get_dto: InventoryGetDto = InventoryGetDto::new("Test".to_string(), 1, None);
    return Json(inventory_get_dto)
}

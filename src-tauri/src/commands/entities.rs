use crate::entities::{CustomEntitiesManager, CustomMappingsManager, Entity, EntityRegistry};
use crate::errors::{Result, SagaError};
use log::info;
use std::collections::HashMap;
use std::fs;
use tauri::AppHandle;

/// Load entities from a JSON file and return them as a map
#[tauri::command]
pub fn load_entities(path: String) -> Result<HashMap<String, Entity>> {
    info!("Loading entities from: {}", path);

    let content = fs::read_to_string(&path)?;
    let mut registry = EntityRegistry::new();
    registry.load_from_str(&content).map_err(SagaError::Validation)?;

    Ok(registry.to_map().clone())
}

/// Get a single entity by name from a loaded entity file
#[tauri::command]
pub fn get_entity(path: String, name: String) -> Result<Option<Entity>> {
    let content = fs::read_to_string(&path)?;

    let mut registry = EntityRegistry::new();
    registry.load_from_str(&content).map_err(SagaError::Validation)?;

    Ok(registry.get(&name).cloned())
}

/// List all entity names from a loaded entity file
#[tauri::command]
pub fn list_entity_names(path: String) -> Result<Vec<String>> {
    let content = fs::read_to_string(&path)?;

    let mut registry = EntityRegistry::new();
    registry.load_from_str(&content).map_err(SagaError::Validation)?;

    Ok(registry.names().into_iter().cloned().collect())
}

/// Load custom entity mappings from the app data directory
#[tauri::command]
pub fn load_custom_mappings(app: AppHandle) -> Result<HashMap<String, String>> {
    let manager = CustomMappingsManager::new(&app).map_err(SagaError::Internal)?;
    Ok(manager.load())
}

/// Save a custom entity mapping
#[tauri::command]
pub fn save_entity_mapping(
    app: AppHandle,
    entity: String,
    translation: String,
) -> Result<()> {
    info!("Saving custom mapping: {} -> {}", entity, translation);
    let manager = CustomMappingsManager::new(&app).map_err(SagaError::Internal)?;
    manager.save(&entity, &translation).map_err(SagaError::Internal)
}

/// Remove a custom entity mapping
#[tauri::command]
pub fn remove_entity_mapping(app: AppHandle, entity: String) -> Result<()> {
    info!("Removing custom mapping: {}", entity);
    let manager = CustomMappingsManager::new(&app).map_err(SagaError::Internal)?;
    manager.remove(&entity).map_err(SagaError::Internal)
}

/// Clear all custom entity mappings
#[tauri::command]
pub fn clear_custom_mappings(app: AppHandle) -> Result<()> {
    info!("Clearing all custom mappings");
    let manager = CustomMappingsManager::new(&app).map_err(SagaError::Internal)?;
    manager.clear().map_err(SagaError::Internal)
}

// ============================================================================
// Custom Entity Definition Commands (not just mappings - full entity CRUD)
// ============================================================================

/// Load all custom entity definitions from the app data directory
#[tauri::command]
pub fn load_custom_entities(app: AppHandle) -> Result<HashMap<String, Entity>> {
    let manager = CustomEntitiesManager::new(&app).map_err(SagaError::Internal)?;
    Ok(manager.load())
}

/// Save a custom entity definition
#[tauri::command(rename_all = "camelCase")]
pub fn save_custom_entity(
    app: AppHandle,
    name: String,
    unicode: String,
    char_value: String,
    description: String,
    category: String,
) -> Result<()> {
    info!("Saving custom entity: {}", name);
    let manager = CustomEntitiesManager::new(&app).map_err(SagaError::Internal)?;
    let entity = Entity {
        unicode,
        char: char_value,
        description,
        category,
    };
    manager.save(&name, entity).map_err(SagaError::Internal)
}

/// Remove a custom entity definition
#[tauri::command]
pub fn remove_custom_entity(app: AppHandle, name: String) -> Result<()> {
    info!("Removing custom entity: {}", name);
    let manager = CustomEntitiesManager::new(&app).map_err(SagaError::Internal)?;
    manager.remove(&name).map_err(SagaError::Internal)
}

/// Clear all custom entity definitions
#[tauri::command]
pub fn clear_custom_entities(app: AppHandle) -> Result<()> {
    info!("Clearing all custom entities");
    let manager = CustomEntitiesManager::new(&app).map_err(SagaError::Internal)?;
    manager.clear().map_err(SagaError::Internal)
}

/// Validate an entity name (alphanumeric + underscore, must start with letter)
#[tauri::command]
pub fn validate_entity_name(name: String) -> bool {
    CustomEntitiesManager::is_valid_name(&name)
}

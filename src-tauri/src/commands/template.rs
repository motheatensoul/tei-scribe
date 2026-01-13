use crate::errors::{Result, SagaError};
use crate::template::{Template, TemplateManager};
use tauri::AppHandle;

#[tauri::command]
pub fn list_templates(app: AppHandle) -> Result<Vec<Template>> {
    let manager = TemplateManager::new(&app)?;
    manager.list_templates()
}

#[tauri::command]
pub fn get_template(app: AppHandle, id: String) -> Result<Template> {
    let manager = TemplateManager::new(&app)?;
    manager.get_template(&id)
}

#[tauri::command]
pub fn save_template(app: AppHandle, template: Template) -> Result<()> {
    let manager = TemplateManager::new(&app)?;
    manager.save_template(&template)
}

#[tauri::command]
pub fn delete_template(app: AppHandle, id: String) -> Result<()> {
    // Prevent deletion of built-in templates
    if id == "tei-p5" || id == "menota" {
        return Err(SagaError::Template("Cannot delete built-in templates".to_string()));
    }
    let manager = TemplateManager::new(&app)?;
    manager.delete_template(&id)
}

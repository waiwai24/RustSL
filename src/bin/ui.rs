use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FilePickerState {
    pub path: Option<PathBuf>,
    pub display_name: Option<String>,
}

impl Default for FilePickerState {
    fn default() -> Self {
        FilePickerState {
            path: None,
            display_name: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ComboBoxState {
    pub selected_index: usize,
    pub items: Vec<ComboBoxItem>,
}

#[derive(Debug, Clone)]
pub struct ComboBoxItem {
    pub label: String,
    pub value: String,
}

impl ComboBoxState {
    pub fn new(items: Vec<ComboBoxItem>) -> Self {
        ComboBoxState {
            selected_index: 0,
            items,
        }
    }
    
    pub fn get_selected_value(&self) -> Option<String> {
        self.items.get(self.selected_index).map(|i| i.value.clone())
    }
    
    pub fn get_selected_label(&self) -> Option<String> {
        self.items.get(self.selected_index).map(|i| i.label.clone())
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextInputState {
    pub value: String,
    pub placeholder: String,
}

impl TextInputState {
    pub fn new(placeholder: String) -> Self {
        TextInputState {
            value: String::new(),
            placeholder,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckboxState {
    pub id: String,
    pub label: String,
    pub checked: bool,
}

impl CheckboxState {
    pub fn new(id: String, label: String) -> Self {
        CheckboxState {
            id,
            label,
            checked: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CheckboxGridState {
    pub items: Vec<CheckboxState>,
}

impl CheckboxGridState {
    pub fn new(items: Vec<CheckboxState>) -> Self {
        CheckboxGridState { items }
    }
    
    pub fn get_checked_ids(&self) -> Vec<String> {
        self.items
            .iter()
            .filter(|cb| cb.checked)
            .map(|cb| cb.id.clone())
            .collect()
    }
}

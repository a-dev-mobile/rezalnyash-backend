// margins.rs - Модуль для работы с отступами при раскрое
//
// Предоставляет структуру для определения отступов от краев листа материала.

use serde::{Deserialize, Serialize};

/// Структура для определения отступов от краев листа
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Margins {
    /// Отступ сверху (в пикселях)
    pub top: u32,
    
    /// Отступ справа (в пикселях)
    pub right: u32,
    
    /// Отступ снизу (в пикселях)
    pub bottom: u32,
    
    /// Отступ слева (в пикселях)
    pub left: u32,
}

impl Default for Margins {
    /// Создает экземпляр Margins со значениями по умолчанию
    fn default() -> Self {
        Margins {
            top: 20,
            right: 20,
            bottom: 20,
            left: 20,
        }
    }
}

impl Margins {
    /// Создает новый экземпляр Margins с заданными значениями
    pub fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Margins {
            top,
            right,
            bottom,
            left,
        }
    }
    
    /// Создает экземпляр Margins с одинаковыми значениями для всех сторон
    pub fn uniform(value: u32) -> Self {
        Margins {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
    
    /// Возвращает максимальное значение отступа из всех сторон
    pub fn max_margin(&self) -> u32 {
        std::cmp::max(
            std::cmp::max(self.top, self.bottom),
            std::cmp::max(self.left, self.right)
        )
    }
}
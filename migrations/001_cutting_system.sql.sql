
-- Таблица материалов
CREATE TABLE IF NOT EXISTS materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_type VARCHAR(50) NOT NULL,
    name_ru VARCHAR(100) NOT NULL,
    name_en VARCHAR(100),
    thickness DECIMAL(8,2),
    color VARCHAR(100),
    grain_direction BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Таблица стандартных размеров для материалов
CREATE TABLE IF NOT EXISTS material_standard_sizes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_id UUID REFERENCES materials(id) ON DELETE CASCADE,
    width DECIMAL(10,2) NOT NULL,
    height DECIMAL(10,2) NOT NULL,
    name VARCHAR(100),
    common_usage TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Таблица расчетов раскроя
CREATE TABLE IF NOT EXISTS cutting_calculations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_name VARCHAR(255),
    status VARCHAR(50) DEFAULT 'processing', -- processing, completed, failed
    settings JSONB NOT NULL, -- настройки раскроя
    input_data JSONB NOT NULL, -- исходные данные (листы и детали)
    result_data JSONB, -- результаты расчета
    statistics JSONB, -- статистика
    processing_time_ms INTEGER,
    error_message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Индексы для производительности
CREATE INDEX IF NOT EXISTS idx_materials_type ON materials(material_type);
CREATE INDEX IF NOT EXISTS idx_calculations_status ON cutting_calculations(status);
CREATE INDEX IF NOT EXISTS idx_calculations_created ON cutting_calculations(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_calculations_project ON cutting_calculations(project_name);

-- Вставка базовых материалов
INSERT INTO materials (material_type, name_ru, name_en, thickness, grain_direction) VALUES
('ЛДСП', 'ЛДСП (Laminated Chipboard)', 'Laminated Chipboard', 16.0, true),
('ДСП', 'ДСП (Chipboard)', 'Chipboard', 16.0, false),
('МДФ', 'МДФ (MDF)', 'MDF', 16.0, false),
('Фанера', 'Фанера (Plywood)', 'Plywood', 18.0, true),
('ДВП', 'ДВП (Fiberboard)', 'Fiberboard', 3.0, false)
ON CONFLICT DO NOTHING;

-- Стандартные размеры листов
INSERT INTO material_standard_sizes (material_id, width, height, name, common_usage)
SELECT 
    m.id,
    sizes.width,
    sizes.height,
    sizes.name,
    sizes.usage
FROM materials m
CROSS JOIN (
    VALUES 
        (2750, 1830, 'Стандартный лист', 'Мебельное производство'),
        (2440, 1220, 'Европейский стандарт', 'Мебель, строительство'),
        (3050, 2070, 'Большой лист', 'Крупные конструкции'),
        (2100, 1400, 'Средний лист', 'Малая мебель')
) AS sizes(width, height, name, usage)
WHERE m.material_type IN ('ЛДСП', 'ДСП', 'МДФ')
ON CONFLICT DO NOTHING;
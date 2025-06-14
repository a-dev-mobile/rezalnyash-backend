-- Миграция: переход на UUID вместо автоинкрементных ID
-- UUID генерируются на бэкенде и передаются в базу

-- Удаление старой схемы (если нужно)
DROP SCHEMA IF EXISTS materials CASCADE;

-- Создание отдельной схемы для материалов (PostgreSQL)
CREATE SCHEMA IF NOT EXISTS materials;

-- Включаем расширение для UUID (если не включено)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Таблица типов материалов
CREATE TABLE materials.material_types (
    material_type_uid UUID PRIMARY KEY,
    name_ru VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Таблица направлений волокон
CREATE TABLE materials.grain_directions (
    grain_direction_uid UUID PRIMARY KEY,
    name_ru VARCHAR(100) NOT NULL,
    name_en VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Таблица ширины
CREATE TABLE materials.widths (
    width_uid UUID PRIMARY KEY,
    width DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Таблица высоты
CREATE TABLE materials.heights (
    height_uid UUID PRIMARY KEY,
    height DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Таблица толщины
CREATE TABLE materials.thicknesses (
    thickness_uid UUID PRIMARY KEY,
    thickness DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Таблица названий материалов
CREATE TABLE materials.material_names (
    material_name_uid UUID PRIMARY KEY,
    name_ru VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Таблица областей применения
CREATE TABLE materials.common_usages (
    common_usage_uid UUID PRIMARY KEY,
    common_usage_ru TEXT NOT NULL,
    common_usage_en TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Основная таблица материалов (связывает все таблицы)
CREATE TABLE materials.materials (
    material_uid UUID PRIMARY KEY,
    material_type_uid UUID NOT NULL,
    grain_direction_uid UUID,
    width_uid UUID,
    height_uid UUID,
    thickness_uid UUID,
    material_name_uid UUID,
    common_usage_uid UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    FOREIGN KEY (material_type_uid) REFERENCES materials.material_types(material_type_uid),
    FOREIGN KEY (grain_direction_uid) REFERENCES materials.grain_directions(grain_direction_uid),
    FOREIGN KEY (width_uid) REFERENCES materials.widths(width_uid),
    FOREIGN KEY (height_uid) REFERENCES materials.heights(height_uid),
    FOREIGN KEY (thickness_uid) REFERENCES materials.thicknesses(thickness_uid),
    FOREIGN KEY (material_name_uid) REFERENCES materials.material_names(material_name_uid),
    FOREIGN KEY (common_usage_uid) REFERENCES materials.common_usages(common_usage_uid)
);

-- Индексы для оптимизации запросов
CREATE INDEX idx_materials_type ON materials.materials(material_type_uid);
CREATE INDEX idx_materials_grain ON materials.materials(grain_direction_uid);
CREATE INDEX idx_materials_width ON materials.materials(width_uid);
CREATE INDEX idx_materials_height ON materials.materials(height_uid);
CREATE INDEX idx_materials_thickness ON materials.materials(thickness_uid);
CREATE INDEX idx_materials_name ON materials.materials(material_name_uid);
CREATE INDEX idx_materials_usage ON materials.materials(common_usage_uid);

-- Индексы для поиска по именам
CREATE INDEX idx_material_types_name_ru ON materials.material_types(name_ru);
CREATE INDEX idx_material_types_name_en ON materials.material_types(name_en);
CREATE INDEX idx_material_names_name_ru ON materials.material_names(name_ru);
CREATE INDEX idx_material_names_name_en ON materials.material_names(name_en);

-- Индексы для поиска по размерам
CREATE INDEX idx_widths_value ON materials.widths(width);
CREATE INDEX idx_heights_value ON materials.heights(height);
CREATE INDEX idx_thicknesses_value ON materials.thicknesses(thickness);

-- Уникальные ограничения для предотвращения дублирования
ALTER TABLE materials.material_types ADD CONSTRAINT uk_material_types_names UNIQUE(name_ru, name_en);
ALTER TABLE materials.grain_directions ADD CONSTRAINT uk_grain_directions_names UNIQUE(name_ru, name_en);
ALTER TABLE materials.material_names ADD CONSTRAINT uk_material_names_names UNIQUE(name_ru, name_en);
ALTER TABLE materials.common_usages ADD CONSTRAINT uk_common_usages_names UNIQUE(common_usage_ru, common_usage_en);
ALTER TABLE materials.widths ADD CONSTRAINT uk_widths_value UNIQUE(width);
ALTER TABLE materials.heights ADD CONSTRAINT uk_heights_value UNIQUE(height);
ALTER TABLE materials.thicknesses ADD CONSTRAINT uk_thicknesses_value UNIQUE(thickness);

-- Триггеры для автоматического обновления updated_at
CREATE OR REPLACE FUNCTION materials.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_material_types_updated_at BEFORE UPDATE ON materials.material_types FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();
CREATE TRIGGER update_grain_directions_updated_at BEFORE UPDATE ON materials.grain_directions FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();
CREATE TRIGGER update_widths_updated_at BEFORE UPDATE ON materials.widths FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();
CREATE TRIGGER update_heights_updated_at BEFORE UPDATE ON materials.heights FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();
CREATE TRIGGER update_thicknesses_updated_at BEFORE UPDATE ON materials.thicknesses FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();
CREATE TRIGGER update_material_names_updated_at BEFORE UPDATE ON materials.material_names FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();
CREATE TRIGGER update_common_usages_updated_at BEFORE UPDATE ON materials.common_usages FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();
CREATE TRIGGER update_materials_updated_at BEFORE UPDATE ON materials.materials FOR EACH ROW EXECUTE FUNCTION materials.update_updated_at_column();

-- Примеры данных с UUID (эти UUID будут генерироваться на бэкенде)

-- Типы материалов
INSERT INTO materials.material_types (material_type_uid, name_ru, name_en) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'Древесина', 'Wood'),
('550e8400-e29b-41d4-a716-446655440002', 'Фанера', 'Plywood'),
('550e8400-e29b-41d4-a716-446655440003', 'МДФ', 'MDF'),
('550e8400-e29b-41d4-a716-446655440004', 'ДСП', 'Chipboard'),
('550e8400-e29b-41d4-a716-446655440005', 'Металл', 'Metal'),
('550e8400-e29b-41d4-a716-446655440006', 'Пластик', 'Plastic');

-- Направления волокон
INSERT INTO materials.grain_directions (grain_direction_uid, name_ru, name_en) VALUES
('660e8400-e29b-41d4-a716-446655440001', 'Вдоль волокон', 'Along grain'),
('660e8400-e29b-41d4-a716-446655440002', 'Поперек волокон', 'Across grain'),
('660e8400-e29b-41d4-a716-446655440003', 'Смешанное', 'Mixed'),
('660e8400-e29b-41d4-a716-446655440004', 'Не применимо', 'Not applicable');

-- Ширина
INSERT INTO materials.widths (width_uid, width) VALUES
('770e8400-e29b-41d4-a716-446655440001', 2000.0),
('770e8400-e29b-41d4-a716-446655440002', 2500.0),
('770e8400-e29b-41d4-a716-446655440003', 2440.0),
('770e8400-e29b-41d4-a716-446655440004', 2800.0),
('770e8400-e29b-41d4-a716-446655440005', 1000.0);

-- Высота
INSERT INTO materials.heights (height_uid, height) VALUES
('880e8400-e29b-41d4-a716-446655440001', 1000.0),
('880e8400-e29b-41d4-a716-446655440002', 1200.0),
('880e8400-e29b-41d4-a716-446655440003', 1220.0),
('880e8400-e29b-41d4-a716-446655440004', 2070.0),
('880e8400-e29b-41d4-a716-446655440005', 500.0);

-- Толщина
INSERT INTO materials.thicknesses (thickness_uid, thickness) VALUES
('990e8400-e29b-41d4-a716-446655440001', 18.0),
('990e8400-e29b-41d4-a716-446655440002', 22.0),
('990e8400-e29b-41d4-a716-446655440003', 15.0),
('990e8400-e29b-41d4-a716-446655440004', 16.0),
('990e8400-e29b-41d4-a716-446655440005', 1.5),
('990e8400-e29b-41d4-a716-446655440006', 12.0),
('990e8400-e29b-41d4-a716-446655440007', 25.0);

-- Названия материалов
INSERT INTO materials.material_names (material_name_uid, name_ru, name_en) VALUES
('aa0e8400-e29b-41d4-a716-446655440001', 'Сосна обрезная', 'Pine lumber'),
('aa0e8400-e29b-41d4-a716-446655440002', 'Дуб массив', 'Oak solid wood'),
('aa0e8400-e29b-41d4-a716-446655440003', 'Фанера березовая', 'Birch plywood'),
('aa0e8400-e29b-41d4-a716-446655440004', 'МДФ ламинированный', 'Laminated MDF'),
('aa0e8400-e29b-41d4-a716-446655440005', 'Лист стальной', 'Steel sheet'),
('aa0e8400-e29b-41d4-a716-446655440006', 'Пластиковый лист', 'Plastic sheet');

-- Области применения
INSERT INTO materials.common_usages (common_usage_uid, common_usage_ru, common_usage_en) VALUES
('bb0e8400-e29b-41d4-a716-446655440001', 'Изготовление мебели, строительство', 'Furniture making, construction'),
('bb0e8400-e29b-41d4-a716-446655440002', 'Элитная мебель, паркет', 'Premium furniture, parquet'),
('bb0e8400-e29b-41d4-a716-446655440003', 'Мебель, упаковка', 'Furniture, packaging'),
('bb0e8400-e29b-41d4-a716-446655440004', 'Корпусная мебель', 'Cabinet furniture'),
('bb0e8400-e29b-41d4-a716-446655440005', 'Металлоконструкции', 'Metal structures'),
('bb0e8400-e29b-41d4-a716-446655440006', 'Реклама, вывески', 'Advertising, signage');

-- Примеры материалов
INSERT INTO materials.materials (
    material_uid,
    material_type_uid, 
    grain_direction_uid, 
    width_uid, 
    height_uid, 
    thickness_uid, 
    material_name_uid, 
    common_usage_uid
) VALUES
('cc0e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440001', '660e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440001', '880e8400-e29b-41d4-a716-446655440001', '990e8400-e29b-41d4-a716-446655440001', 'aa0e8400-e29b-41d4-a716-446655440001', 'bb0e8400-e29b-41d4-a716-446655440001'), -- Сосна 2000x1000x18
('cc0e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440001', '660e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440002', '880e8400-e29b-41d4-a716-446655440002', '990e8400-e29b-41d4-a716-446655440002', 'aa0e8400-e29b-41d4-a716-446655440002', 'bb0e8400-e29b-41d4-a716-446655440002'), -- Дуб 2500x1200x22
('cc0e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440002', '660e8400-e29b-41d4-a716-446655440003', '770e8400-e29b-41d4-a716-446655440003', '880e8400-e29b-41d4-a716-446655440003', '990e8400-e29b-41d4-a716-446655440003', 'aa0e8400-e29b-41d4-a716-446655440003', 'bb0e8400-e29b-41d4-a716-446655440003'), -- Фанера 2440x1220x15
('cc0e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440003', '660e8400-e29b-41d4-a716-446655440004', '770e8400-e29b-41d4-a716-446655440004', '880e8400-e29b-41d4-a716-446655440004', '990e8400-e29b-41d4-a716-446655440004', 'aa0e8400-e29b-41d4-a716-446655440004', 'bb0e8400-e29b-41d4-a716-446655440004'), -- МДФ 2800x2070x16
('cc0e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440005', '660e8400-e29b-41d4-a716-446655440004', '770e8400-e29b-41d4-a716-446655440001', '880e8400-e29b-41d4-a716-446655440001', '990e8400-e29b-41d4-a716-446655440005', 'aa0e8400-e29b-41d4-a716-446655440005', 'bb0e8400-e29b-41d4-a716-446655440005'); -- Сталь 2000x1000x1.5

-- VIEW для удобного просмотра всех материалов
CREATE VIEW materials.v_materials AS
SELECT 
    m.material_uid,
    mt.material_type_uid,
    mt.name_ru as material_type_ru,
    mt.name_en as material_type_en,
    mn.material_name_uid,
    mn.name_ru as material_name_ru,
    mn.name_en as material_name_en,
    w.width_uid,
    w.width,
    h.height_uid,
    h.height,
    t.thickness_uid,
    t.thickness,
    gd.grain_direction_uid,
    gd.name_ru as grain_direction_ru,
    gd.name_en as grain_direction_en,
    cu.common_usage_uid,
    cu.common_usage_ru,
    cu.common_usage_en,
    m.created_at,
    m.updated_at
FROM materials.materials m
LEFT JOIN materials.material_types mt ON m.material_type_uid = mt.material_type_uid
LEFT JOIN materials.material_names mn ON m.material_name_uid = mn.material_name_uid
LEFT JOIN materials.widths w ON m.width_uid = w.width_uid
LEFT JOIN materials.heights h ON m.height_uid = h.height_uid
LEFT JOIN materials.thicknesses t ON m.thickness_uid = t.thickness_uid
LEFT JOIN materials.grain_directions gd ON m.grain_direction_uid = gd.grain_direction_uid
LEFT JOIN materials.common_usages cu ON m.common_usage_uid = cu.common_usage_uid;

-- VIEW для работы с размерами
CREATE VIEW materials.v_dimensions AS
SELECT 
    w.width_uid,
    w.width,
    h.height_uid,
    h.height,
    t.thickness_uid,
    t.thickness,
    CONCAT(w.width, 'x', h.height, 'x', t.thickness) as dimension_string
FROM materials.widths w
CROSS JOIN materials.heights h
CROSS JOIN materials.thicknesses t
ORDER BY w.width, h.height, t.thickness;

-- VIEW для справочников (все локализованные данные)
CREATE VIEW materials.v_references AS
SELECT 
    'material_types' as table_name,
    material_type_uid::text as uid,
    name_ru,
    name_en,
    NULL as description_ru,
    NULL as description_en,
    created_at,
    updated_at
FROM materials.material_types

UNION ALL

SELECT 
    'grain_directions' as table_name,
    grain_direction_uid::text as uid,
    name_ru,
    name_en,
    NULL as description_ru,
    NULL as description_en,
    created_at,
    updated_at
FROM materials.grain_directions

UNION ALL

SELECT 
    'material_names' as table_name,
    material_name_uid::text as uid,
    name_ru,
    name_en,
    NULL as description_ru,
    NULL as description_en,
    created_at,
    updated_at
FROM materials.material_names

UNION ALL

SELECT 
    'common_usages' as table_name,
    common_usage_uid::text as uid,
    common_usage_ru as name_ru,
    common_usage_en as name_en,
    NULL as description_ru,
    NULL as description_en,
    created_at,
    updated_at
FROM materials.common_usages;

-- Функция для добавления нового материала (UUID генерируются на бэкенде)
CREATE OR REPLACE FUNCTION materials.add_material(
    p_material_uid UUID,
    p_material_type_uid UUID,
    p_material_type_ru VARCHAR(255),
    p_material_type_en VARCHAR(255),
    p_material_name_uid UUID,
    p_material_name_ru VARCHAR(255),
    p_material_name_en VARCHAR(255),
    p_width_uid UUID,
    p_width DOUBLE PRECISION,
    p_height_uid UUID,
    p_height DOUBLE PRECISION,
    p_thickness_uid UUID,
    p_thickness DOUBLE PRECISION,
    p_grain_direction_uid UUID,
    p_grain_direction_ru VARCHAR(100),
    p_grain_direction_en VARCHAR(100),
    p_common_usage_uid UUID,
    p_common_usage_ru TEXT,
    p_common_usage_en TEXT
) RETURNS UUID AS $$
DECLARE
    v_material_type_exists BOOLEAN;
    v_material_name_exists BOOLEAN;
    v_width_exists BOOLEAN;
    v_height_exists BOOLEAN;
    v_thickness_exists BOOLEAN;
    v_grain_direction_exists BOOLEAN;
    v_common_usage_exists BOOLEAN;
BEGIN
    -- Проверить существование типа материала
    SELECT EXISTS(SELECT 1 FROM materials.material_types WHERE material_type_uid = p_material_type_uid) INTO v_material_type_exists;
    
    IF NOT v_material_type_exists THEN
        INSERT INTO materials.material_types (material_type_uid, name_ru, name_en) 
        VALUES (p_material_type_uid, p_material_type_ru, p_material_type_en);
    END IF;
    
    -- Проверить существование названия материала
    SELECT EXISTS(SELECT 1 FROM materials.material_names WHERE material_name_uid = p_material_name_uid) INTO v_material_name_exists;
    
    IF NOT v_material_name_exists THEN
        INSERT INTO materials.material_names (material_name_uid, name_ru, name_en) 
        VALUES (p_material_name_uid, p_material_name_ru, p_material_name_en);
    END IF;
    
    -- Проверить существование ширины
    SELECT EXISTS(SELECT 1 FROM materials.widths WHERE width_uid = p_width_uid) INTO v_width_exists;
    IF NOT v_width_exists THEN
        INSERT INTO materials.widths (width_uid, width) VALUES (p_width_uid, p_width);
    END IF;
    
    -- Проверить существование высоты
    SELECT EXISTS(SELECT 1 FROM materials.heights WHERE height_uid = p_height_uid) INTO v_height_exists;
    IF NOT v_height_exists THEN
        INSERT INTO materials.heights (height_uid, height) VALUES (p_height_uid, p_height);
    END IF;
    
    -- Проверить существование толщины
    SELECT EXISTS(SELECT 1 FROM materials.thicknesses WHERE thickness_uid = p_thickness_uid) INTO v_thickness_exists;
    IF NOT v_thickness_exists THEN
        INSERT INTO materials.thicknesses (thickness_uid, thickness) VALUES (p_thickness_uid, p_thickness);
    END IF;
    
    -- Проверить существование направления волокон
    SELECT EXISTS(SELECT 1 FROM materials.grain_directions WHERE grain_direction_uid = p_grain_direction_uid) INTO v_grain_direction_exists;
    
    IF NOT v_grain_direction_exists THEN
        INSERT INTO materials.grain_directions (grain_direction_uid, name_ru, name_en) 
        VALUES (p_grain_direction_uid, p_grain_direction_ru, p_grain_direction_en);
    END IF;
    
    -- Проверить существование области применения
    SELECT EXISTS(SELECT 1 FROM materials.common_usages WHERE common_usage_uid = p_common_usage_uid) INTO v_common_usage_exists;
    
    IF NOT v_common_usage_exists THEN
        INSERT INTO materials.common_usages (common_usage_uid, common_usage_ru, common_usage_en) 
        VALUES (p_common_usage_uid, p_common_usage_ru, p_common_usage_en);
    END IF;
    
    -- Создать материал
    INSERT INTO materials.materials (
        material_uid,
        material_type_uid, 
        grain_direction_uid, 
        width_uid, 
        height_uid, 
        thickness_uid, 
        material_name_uid, 
        common_usage_uid
    ) VALUES (
        p_material_uid,
        p_material_type_uid,
        p_grain_direction_uid,
        p_width_uid,
        p_height_uid,
        p_thickness_uid,
        p_material_name_uid,
        p_common_usage_uid
    );
    
    RETURN p_material_uid;
END;
$$ LANGUAGE plpgsql;

-- Примеры использования с UUID:

-- Просмотр всех материалов
-- SELECT * FROM materials.v_materials;

-- Просмотр материала по UUID
-- SELECT * FROM materials.v_materials WHERE material_uid = 'cc0e8400-e29b-41d4-a716-446655440001';

-- Поиск материалов по типу
-- SELECT * FROM materials.v_materials WHERE material_type_ru = 'Древесина';

-- Поиск по размерам
-- SELECT * FROM materials.v_materials WHERE width >= 2000 AND thickness <= 20;

-- Добавление нового материала (все UUID генерируются на бэкенде)
/*
SELECT materials.add_material(
    'dd0e8400-e29b-41d4-a716-446655440001'::UUID, -- material_uid
    '550e8400-e29b-41d4-a716-446655440001'::UUID, -- material_type_uid (существующий)
    'Древесина', 'Wood', -- тип материала
    'ee0e8400-e29b-41d4-a716-446655440001'::UUID, -- material_name_uid
    'Береза', 'Birch', -- название материала
    'ff0e8400-e29b-41d4-a716-446655440001'::UUID, -- width_uid
    1800.0, -- ширина
    '110e8400-e29b-41d4-a716-446655440001'::UUID, -- height_uid
    900.0, -- высота
    '220e8400-e29b-41d4-a716-446655440001'::UUID, -- thickness_uid
    20.0, -- толщина
    '660e8400-e29b-41d4-a716-446655440001'::UUID, -- grain_direction_uid (существующий)
    'Вдоль волокон', 'Along grain', -- направление волокон
    '330e8400-e29b-41d4-a716-446655440001'::UUID, -- common_usage_uid
    'Мебель', 'Furniture' -- область применения
);
*/

-- Просмотр всех справочников
-- SELECT * FROM materials.v_references WHERE table_name = 'material_types';

-- Получение всех доступных размеров
-- SELECT DISTINCT width FROM materials.widths ORDER BY width;
-- SELECT DISTINCT height FROM materials.heights ORDER BY height;
-- SELECT DISTINCT thickness FROM materials.thicknesses ORDER BY thickness;
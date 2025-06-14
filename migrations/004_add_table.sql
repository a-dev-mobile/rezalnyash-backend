-- Создание отдельной схемы для материалов (PostgreSQL)
CREATE SCHEMA IF NOT EXISTS materials;

-- Таблица типов материалов
CREATE TABLE materials.material_types (
    material_type_id SERIAL PRIMARY KEY,
    name_ru VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL
);

-- Таблица направлений волокон
CREATE TABLE materials.grain_directions (
    grain_direction_id SERIAL PRIMARY KEY,
    name_ru VARCHAR(100) NOT NULL,
    name_en VARCHAR(100) NOT NULL
);

-- Таблица ширины
CREATE TABLE materials.widths (
    width_id SERIAL PRIMARY KEY,
    width DOUBLE PRECISION NOT NULL
);

-- Таблица высоты
CREATE TABLE materials.heights (
    height_id SERIAL PRIMARY KEY,
    height DOUBLE PRECISION NOT NULL
);

-- Таблица толщины
CREATE TABLE materials.thicknesses (
    thickness_id SERIAL PRIMARY KEY,
    thickness DOUBLE PRECISION NOT NULL
);

-- Таблица названий материалов
CREATE TABLE materials.material_names (
    material_name_id SERIAL PRIMARY KEY,
    name_ru VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL
);

-- Таблица областей применения
CREATE TABLE materials.common_usages (
    common_usage_id SERIAL PRIMARY KEY,
    common_usage_ru TEXT NOT NULL,
    common_usage_en TEXT NOT NULL
);

-- Основная таблица материалов (связывает все таблицы)
CREATE TABLE materials.materials (
    material_id SERIAL PRIMARY KEY,
    material_type_id INTEGER NOT NULL,
    grain_direction_id INTEGER,
    width_id INTEGER,
    height_id INTEGER,
    thickness_id INTEGER,
    material_name_id INTEGER,
    common_usage_id INTEGER,
    
    FOREIGN KEY (material_type_id) REFERENCES materials.material_types(material_type_id),
    FOREIGN KEY (grain_direction_id) REFERENCES materials.grain_directions(grain_direction_id),
    FOREIGN KEY (width_id) REFERENCES materials.widths(width_id),
    FOREIGN KEY (height_id) REFERENCES materials.heights(height_id),
    FOREIGN KEY (thickness_id) REFERENCES materials.thicknesses(thickness_id),
    FOREIGN KEY (material_name_id) REFERENCES materials.material_names(material_name_id),
    FOREIGN KEY (common_usage_id) REFERENCES materials.common_usages(common_usage_id)
);

-- Индексы
CREATE INDEX idx_materials_type ON materials.materials(material_type_id);
CREATE INDEX idx_materials_grain ON materials.materials(grain_direction_id);
CREATE INDEX idx_materials_width ON materials.materials(width_id);
CREATE INDEX idx_materials_height ON materials.materials(height_id);
CREATE INDEX idx_materials_thickness ON materials.materials(thickness_id);
CREATE INDEX idx_materials_name ON materials.materials(material_name_id);
CREATE INDEX idx_materials_usage ON materials.materials(common_usage_id);

-- Примеры данных

-- Типы материалов
INSERT INTO materials.material_types (name_ru, name_en) VALUES
('Древесина', 'Wood'),
('Фанера', 'Plywood'),
('МДФ', 'MDF'),
('ДСП', 'Chipboard'),
('Металл', 'Metal'),
('Пластик', 'Plastic');

-- Направления волокон
INSERT INTO materials.grain_directions (name_ru, name_en) VALUES
('Вдоль волокон', 'Along grain'),
('Поперек волокон', 'Across grain'),
('Смешанное', 'Mixed'),
('Не применимо', 'Not applicable');

-- Ширина
INSERT INTO materials.widths (width) VALUES
(2000.0),
(2500.0),
(2440.0),
(2800.0),
(1000.0);

-- Высота
INSERT INTO materials.heights (height) VALUES
(1000.0),
(1200.0),
(1220.0),
(2070.0),
(500.0);

-- Толщина
INSERT INTO materials.thicknesses (thickness) VALUES
(18.0),
(22.0),
(15.0),
(16.0),
(1.5),
(12.0),
(25.0);

-- Названия материалов
INSERT INTO materials.material_names (name_ru, name_en) VALUES
('Сосна обрезная', 'Pine lumber'),
('Дуб массив', 'Oak solid wood'),
('Фанера березовая', 'Birch plywood'),
('МДФ ламинированный', 'Laminated MDF'),
('Лист стальной', 'Steel sheet'),
('Пластиковый лист', 'Plastic sheet');

-- Области применения
INSERT INTO materials.common_usages (common_usage_ru, common_usage_en) VALUES
('Изготовление мебели, строительство', 'Furniture making, construction'),
('Элитная мебель, паркет', 'Premium furniture, parquet'),
('Мебель, упаковка', 'Furniture, packaging'),
('Корпусная мебель', 'Cabinet furniture'),
('Металлоконструкции', 'Metal structures'),
('Реклама, вывески', 'Advertising, signage');

-- Примеры материалов
INSERT INTO materials.materials (
    material_type_id, 
    grain_direction_id, 
    width_id, 
    height_id, 
    thickness_id, 
    material_name_id, 
    common_usage_id
) VALUES
(1, 1, 1, 1, 1, 1, 1), -- Сосна 2000x1000x18
(1, 1, 2, 2, 2, 2, 2), -- Дуб 2500x1200x22
(2, 3, 3, 3, 3, 3, 3), -- Фанера 2440x1220x15
(3, 4, 4, 4, 4, 4, 4), -- МДФ 2800x2070x16
(5, 4, 1, 1, 5, 5, 5); -- Сталь 2000x1000x1.5

-- VIEW для удобного просмотра всех материалов
CREATE VIEW materials.v_materials AS
SELECT 
    m.material_id,
    mt.name_ru as material_type_ru,
    mt.name_en as material_type_en,
    mn.name_ru as material_name_ru,
    mn.name_en as material_name_en,
    w.width,
    h.height,
    t.thickness,
    gd.name_ru as grain_direction_ru,
    gd.name_en as grain_direction_en,
    cu.common_usage_ru,
    cu.common_usage_en
FROM materials.materials m
LEFT JOIN materials.material_types mt ON m.material_type_id = mt.material_type_id
LEFT JOIN materials.material_names mn ON m.material_name_id = mn.material_name_id
LEFT JOIN materials.widths w ON m.width_id = w.width_id
LEFT JOIN materials.heights h ON m.height_id = h.height_id
LEFT JOIN materials.thicknesses t ON m.thickness_id = t.thickness_id
LEFT JOIN materials.grain_directions gd ON m.grain_direction_id = gd.grain_direction_id
LEFT JOIN materials.common_usages cu ON m.common_usage_id = cu.common_usage_id;

-- VIEW для работы с размерами
CREATE VIEW materials.v_dimensions AS
SELECT 
    w.width_id,
    w.width,
    h.height_id,
    h.height,
    t.thickness_id,
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
    material_type_id as id,
    name_ru,
    name_en,
    NULL as description_ru,
    NULL as description_en
FROM materials.material_types

UNION ALL

SELECT 
    'grain_directions' as table_name,
    grain_direction_id as id,
    name_ru,
    name_en,
    NULL as description_ru,
    NULL as description_en
FROM materials.grain_directions

UNION ALL

SELECT 
    'material_names' as table_name,
    material_name_id as id,
    name_ru,
    name_en,
    NULL as description_ru,
    NULL as description_en
FROM materials.material_names

UNION ALL

SELECT 
    'common_usages' as table_name,
    common_usage_id as id,
    common_usage_ru as name_ru,
    common_usage_en as name_en,
    NULL as description_ru,
    NULL as description_en
FROM materials.common_usages;

-- Функция для добавления нового материала (PostgreSQL)
CREATE OR REPLACE FUNCTION materials.add_material(
    p_material_type_ru VARCHAR(255),
    p_material_type_en VARCHAR(255),
    p_material_name_ru VARCHAR(255),
    p_material_name_en VARCHAR(255),
    p_width DOUBLE PRECISION,
    p_height DOUBLE PRECISION,
    p_thickness DOUBLE PRECISION,
    p_grain_direction_ru VARCHAR(100),
    p_grain_direction_en VARCHAR(100),
    p_common_usage_ru TEXT,
    p_common_usage_en TEXT
) RETURNS INTEGER AS $$
DECLARE
    v_material_type_id INTEGER;
    v_material_name_id INTEGER;
    v_width_id INTEGER;
    v_height_id INTEGER;
    v_thickness_id INTEGER;
    v_grain_direction_id INTEGER;
    v_common_usage_id INTEGER;
    v_new_material_id INTEGER;
BEGIN
    -- Получить или создать тип материала
    SELECT material_type_id INTO v_material_type_id 
    FROM materials.material_types 
    WHERE name_ru = p_material_type_ru AND name_en = p_material_type_en;
    
    IF v_material_type_id IS NULL THEN
        INSERT INTO materials.material_types (name_ru, name_en) 
        VALUES (p_material_type_ru, p_material_type_en)
        RETURNING material_type_id INTO v_material_type_id;
    END IF;
    
    -- Получить или создать название материала
    SELECT material_name_id INTO v_material_name_id 
    FROM materials.material_names 
    WHERE name_ru = p_material_name_ru AND name_en = p_material_name_en;
    
    IF v_material_name_id IS NULL THEN
        INSERT INTO materials.material_names (name_ru, name_en) 
        VALUES (p_material_name_ru, p_material_name_en)
        RETURNING material_name_id INTO v_material_name_id;
    END IF;
    
    -- Получить или создать ширину
    SELECT width_id INTO v_width_id FROM materials.widths WHERE width = p_width;
    IF v_width_id IS NULL THEN
        INSERT INTO materials.widths (width) VALUES (p_width)
        RETURNING width_id INTO v_width_id;
    END IF;
    
    -- Получить или создать высоту
    SELECT height_id INTO v_height_id FROM materials.heights WHERE height = p_height;
    IF v_height_id IS NULL THEN
        INSERT INTO materials.heights (height) VALUES (p_height)
        RETURNING height_id INTO v_height_id;
    END IF;
    
    -- Получить или создать толщину
    SELECT thickness_id INTO v_thickness_id FROM materials.thicknesses WHERE thickness = p_thickness;
    IF v_thickness_id IS NULL THEN
        INSERT INTO materials.thicknesses (thickness) VALUES (p_thickness)
        RETURNING thickness_id INTO v_thickness_id;
    END IF;
    
    -- Получить или создать направление волокон
    SELECT grain_direction_id INTO v_grain_direction_id 
    FROM materials.grain_directions 
    WHERE name_ru = p_grain_direction_ru AND name_en = p_grain_direction_en;
    
    IF v_grain_direction_id IS NULL THEN
        INSERT INTO materials.grain_directions (name_ru, name_en) 
        VALUES (p_grain_direction_ru, p_grain_direction_en)
        RETURNING grain_direction_id INTO v_grain_direction_id;
    END IF;
    
    -- Получить или создать область применения
    SELECT common_usage_id INTO v_common_usage_id 
    FROM materials.common_usages 
    WHERE common_usage_ru = p_common_usage_ru AND common_usage_en = p_common_usage_en;
    
    IF v_common_usage_id IS NULL THEN
        INSERT INTO materials.common_usages (common_usage_ru, common_usage_en) 
        VALUES (p_common_usage_ru, p_common_usage_en)
        RETURNING common_usage_id INTO v_common_usage_id;
    END IF;
    
    -- Создать материал
    INSERT INTO materials.materials (
        material_type_id, 
        grain_direction_id, 
        width_id, 
        height_id, 
        thickness_id, 
        material_name_id, 
        common_usage_id
    ) VALUES (
        v_material_type_id,
        v_grain_direction_id,
        v_width_id,
        v_height_id,
        v_thickness_id,
        v_material_name_id,
        v_common_usage_id
    ) RETURNING material_id INTO v_new_material_id;
    
    RETURN v_new_material_id;
END;
$$ LANGUAGE plpgsql;

-- Примеры использования:

-- Просмотр всех материалов
-- SELECT * FROM materials.v_materials;

-- Просмотр размеров
-- SELECT * FROM materials.v_dimensions WHERE width = 2000 AND height = 1000;

-- Поиск материалов по типу
-- SELECT * FROM materials.v_materials WHERE material_type_ru = 'Древесина';

-- Поиск по размерам
-- SELECT * FROM materials.v_materials WHERE width >= 2000 AND thickness <= 20;

-- Добавление нового материала
-- SELECT materials.add_material('Древесина', 'Wood', 'Береза', 'Birch', 1800.0, 900.0, 20.0, 'Вдоль волокон', 'Along grain', 'Мебель', 'Furniture');

-- Просмотр всех справочников
-- SELECT * FROM materials.v_references WHERE table_name = 'material_types';

-- Получение всех доступных размеров
-- SELECT DISTINCT width FROM materials.v_dimensions ORDER BY width;
-- SELECT DISTINCT height FROM materials.v_dimensions ORDER BY height;
-- SELECT DISTINCT thickness FROM materials.v_dimensions ORDER BY thickness;
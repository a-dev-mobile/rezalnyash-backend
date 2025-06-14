-- Удаляем основную таблицу materials (с зависимостями)
DROP TABLE IF EXISTS materials.materials CASCADE;

-- Удаляем справочные таблицы
DROP TABLE IF EXISTS materials.material_types CASCADE;
DROP TABLE IF EXISTS materials.material_names CASCADE;
DROP TABLE IF EXISTS materials.grain_directions CASCADE;
DROP TABLE IF EXISTS materials.widths CASCADE;
DROP TABLE IF EXISTS materials.heights CASCADE;
DROP TABLE IF EXISTS materials.thicknesses CASCADE;
DROP TABLE IF EXISTS materials.common_usages CASCADE;

-- Создаем таблицу типов материалов
CREATE TABLE materials.types (
    type_uid    UUID         NOT NULL PRIMARY KEY,
    name_ru     VARCHAR(255) NOT NULL,
    name_en     VARCHAR(255) NOT NULL,
    CONSTRAINT uk_types_names UNIQUE (name_ru, name_en)
);

-- Индексы для types
CREATE INDEX idx_types_name_ru ON materials.types (name_ru);
CREATE INDEX idx_types_name_en ON materials.types (name_en);

-- Создаем таблицу названий материалов
CREATE TABLE materials.names (
    name_uid    UUID         NOT NULL PRIMARY KEY,
    name_ru     VARCHAR(255) NOT NULL,
    name_en     VARCHAR(255) NOT NULL,
    CONSTRAINT uk_names_names UNIQUE (name_ru, name_en)
);

-- Индексы для names
CREATE INDEX idx_names_name_ru ON materials.names (name_ru);
CREATE INDEX idx_names_name_en ON materials.names (name_en);

-- Создаем таблицу направлений волокон
CREATE TABLE materials.grain_directions (
    grain_direction_uid UUID         NOT NULL PRIMARY KEY,
    name_ru             VARCHAR(100) NOT NULL,
    name_en             VARCHAR(100) NOT NULL,
    CONSTRAINT uk_grain_directions_names UNIQUE (name_ru, name_en)
);

-- Создаем таблицу ширин
CREATE TABLE materials.widths (
    width_uid   UUID             NOT NULL PRIMARY KEY,
    width       DOUBLE PRECISION NOT NULL CONSTRAINT uk_widths_value UNIQUE
);

-- Индекс для widths
CREATE INDEX idx_widths_value ON materials.widths (width);

-- Создаем таблицу высот
CREATE TABLE materials.heights (
    height_uid  UUID             NOT NULL PRIMARY KEY,
    height      DOUBLE PRECISION NOT NULL CONSTRAINT uk_heights_value UNIQUE
);

-- Индекс для heights
CREATE INDEX idx_heights_value ON materials.heights (height);

-- Создаем таблицу толщин
CREATE TABLE materials.thicknesses (
    thickness_uid UUID             NOT NULL PRIMARY KEY,
    thickness     DOUBLE PRECISION NOT NULL CONSTRAINT uk_thicknesses_value UNIQUE
);

-- Индекс для thicknesses
CREATE INDEX idx_thicknesses_value ON materials.thicknesses (thickness);

-- Создаем таблицу общих применений
CREATE TABLE materials.common_usages (
    common_usage_uid UUID NOT NULL PRIMARY KEY,
    common_usage_ru  TEXT NOT NULL,
    common_usage_en  TEXT NOT NULL,
    CONSTRAINT uk_common_usages_names UNIQUE (common_usage_ru, common_usage_en)
);

-- Создаем основную таблицу материалов
CREATE TABLE materials.materials (
    material_uid        UUID NOT NULL PRIMARY KEY,
    type_uid            UUID NOT NULL REFERENCES materials.types(type_uid),
    grain_direction_uid UUID REFERENCES materials.grain_directions(grain_direction_uid),
    width_uid           UUID REFERENCES materials.widths(width_uid),
    height_uid          UUID REFERENCES materials.heights(height_uid),
    thickness_uid       UUID REFERENCES materials.thicknesses(thickness_uid),
    name_uid            UUID REFERENCES materials.names(name_uid),
    common_usage_uid    UUID REFERENCES materials.common_usages(common_usage_uid)
);

-- Индексы для materials
CREATE INDEX idx_materials_type ON materials.materials (type_uid);
CREATE INDEX idx_materials_grain ON materials.materials (grain_direction_uid);
CREATE INDEX idx_materials_width ON materials.materials (width_uid);
CREATE INDEX idx_materials_height ON materials.materials (height_uid);
CREATE INDEX idx_materials_thickness ON materials.materials (thickness_uid);
CREATE INDEX idx_materials_name ON materials.materials (name_uid);
CREATE INDEX idx_materials_usage ON materials.materials (common_usage_uid);

-- Установка владельца таблиц
ALTER TABLE materials.types OWNER TO postgres;
ALTER TABLE materials.names OWNER TO postgres;
ALTER TABLE materials.grain_directions OWNER TO postgres;
ALTER TABLE materials.widths OWNER TO postgres;
ALTER TABLE materials.heights OWNER TO postgres;
ALTER TABLE materials.thicknesses OWNER TO postgres;
ALTER TABLE materials.common_usages OWNER TO postgres;
ALTER TABLE materials.materials OWNER TO postgres;
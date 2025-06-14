-- Создайте новую миграцию для исправления типов

-- Изменяем тип thickness с NUMERIC на DOUBLE PRECISION (FLOAT8)
ALTER TABLE materials 
ALTER COLUMN thickness TYPE DOUBLE PRECISION;

-- Изменяем типы width и height с NUMERIC на DOUBLE PRECISION
ALTER TABLE material_standard_sizes 
ALTER COLUMN width TYPE DOUBLE PRECISION,
ALTER COLUMN height TYPE DOUBLE PRECISION;

-- Обновляем существующие данные (если нужно)
UPDATE materials SET thickness = thickness::DOUBLE PRECISION WHERE thickness IS NOT NULL;
UPDATE material_standard_sizes SET 
    width = width::DOUBLE PRECISION,
    height = height::DOUBLE PRECISION;

-- Добавляем комментарии для ясности
COMMENT ON COLUMN materials.thickness IS 'Толщина материала в миллиметрах (DOUBLE PRECISION)';
COMMENT ON COLUMN material_standard_sizes.width IS 'Ширина листа в миллиметрах (DOUBLE PRECISION)';
COMMENT ON COLUMN material_standard_sizes.height IS 'Высота листа в миллиметрах (DOUBLE PRECISION)';

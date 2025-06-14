
-- FILE: migrations/003_fix_types.sql
-- Исправляем все типы данных одной миграцией



-- 2. Изменяем TIMESTAMP на TIMESTAMPTZ (с часовым поясом)
ALTER TABLE materials 
ALTER COLUMN created_at TYPE TIMESTAMPTZ;

ALTER TABLE material_standard_sizes 
ALTER COLUMN created_at TYPE TIMESTAMPTZ;

ALTER TABLE cutting_calculations 
ALTER COLUMN created_at TYPE TIMESTAMPTZ,
ALTER COLUMN updated_at TYPE TIMESTAMPTZ;

-- 3. Устанавливаем DEFAULT значения с UTC временем
ALTER TABLE materials 
ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE material_standard_sizes 
ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE cutting_calculations 
ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP,
ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;

-- 4. Обновляем существующие данные (конвертируем в UTC)
UPDATE materials 
SET created_at = created_at AT TIME ZONE 'UTC' 
WHERE created_at IS NOT NULL;

UPDATE material_standard_sizes 
SET created_at = created_at AT TIME ZONE 'UTC' 
WHERE created_at IS NOT NULL;

UPDATE cutting_calculations 
SET created_at = created_at AT TIME ZONE 'UTC',
    updated_at = updated_at AT TIME ZONE 'UTC'
WHERE created_at IS NOT NULL;

-- 5. Добавляем комментарии для ясности

COMMENT ON COLUMN materials.created_at IS 'Дата создания записи (TIMESTAMPTZ в UTC)';


COMMENT ON COLUMN material_standard_sizes.created_at IS 'Дата создания записи (TIMESTAMPTZ в UTC)';

COMMENT ON COLUMN cutting_calculations.created_at IS 'Дата создания расчета (TIMESTAMPTZ в UTC)';
COMMENT ON COLUMN cutting_calculations.updated_at IS 'Дата обновления расчета (TIMESTAMPTZ в UTC)';
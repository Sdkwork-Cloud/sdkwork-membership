Locale seed directories:

- `zh-CN/` — default active locale (Simplified Chinese display text)
- `en-US/` — active locale (English display text)
- `ja-JP/`, `de-DE/`, `fr-FR/`, `ru-RU/`, `ko-KR/` — reserved placeholders

Each active locale directory contains ordered SQL seed files referenced by `seeds/seed.manifest.json`.
Locale scripts run AFTER common scripts and override display-facing columns
(name, title, description, grant_quantity text, spec_json tags) via UPDATE statements.
Structural columns (id, tenant_id, benefit_code, price_amount, rank, etc.) are
locale-neutral and set only by `common/001_catalog.sql`.

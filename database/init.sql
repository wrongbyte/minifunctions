-- TODO: improve the tenants table
CREATE TABLE IF NOT EXISTS tenants (
    tenant_id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS function_definitions (
    function_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(tenant_id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    CONSTRAINT unique_tenant_function_name UNIQUE (tenant_id, name)
);

CREATE TABLE IF NOT EXISTS function_versions (
    version_id UUID PRIMARY KEY,
    function_id UUID NOT NULL REFERENCES function_definitions(function_id),
    version INTEGER NOT NULL,
    function_code TEXT NOT NULL,
    timeout_ms INTEGER DEFAULT 30000,
    memory_mb INTEGER DEFAULT 128,
    environment JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    CONSTRAINT unique_function_version UNIQUE (function_id, version)
);

CREATE INDEX IF NOT EXISTS idx_function_definitions_tenant_id ON function_definitions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_function_versions_function_id ON function_versions(function_id);

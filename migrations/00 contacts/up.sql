CREATE TABLE addressbooks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    name VARCHAR NOT NULL
);

CREATE TABLE addressbook_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    addressbook_id UUID NOT NULL REFERENCES addressbooks(id),
    label VARCHAR NOT NULL
);

CREATE TABLE email_addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account VARCHAR NOT NULL,
    domain VARCHAR NOT NULL
);

CREATE TABLE phone_numbers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phone VARCHAR,
    extension VARCHAR NOT NULL
);

CREATE TABLE physical_addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    post_office_box VARCHAR,
    extension VARCHAR,
    street VARCHAR,
    locality VARCHAR,
    region VARCHAR,
    code VARCHAR,
    country VARCHAR
);

CREATE TABLE contacts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    addressbook_id UUID NOT NULL REFERENCES addressbooks(id),
    first_name VARCHAR,
    last_name VARCHAR,
    middle_name VARCHAR,
    name_prefix VARCHAR,
    name_suffix VARCHAR,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE contact_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    tag_id UUID NOT NULL REFERENCES addressbook_tags(id)
);

CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE organization_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    tag_id UUID NOT NULL REFERENCES addressbook_tags(id)
);

CREATE TABLE organization_contacts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL,
    contact_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE organization_phones (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    phone_id UUID NOT NULL REFERENCES phone_numbers(id)
);

CREATE TABLE organization_emails (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    email_id UUID NOT NULL REFERENCES email_addresses(id)
);

CREATE TABLE organization_addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    address_is UUID NOT NULL REFERENCES physical_addresses(id)
);

CREATE TABLE contact_phones (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    phone_id UUID NOT NULL REFERENCES phone_numbers(id),
    phone_type VARCHAR,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE contact_addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    address_id UUID NOT NULL REFERENCES physical_addresses(id),
    address_type VARCHAR
);

CREATE TABLE contact_emails (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    email_id UUID NOT NULL REFERENCES email_addresses(id),
    email_type VARCHAR NOT NULL
);

CREATE TABLE shared_addressbooks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID NOT NULL,
    addressbook_id UUID NOT NULL,
    role_id UUID NOT NULL,
    account_id UUID NOT NULL
);

SELECT diesel_manage_updated_at('contacts');
table! {
    addressbooks (id) {
        id -> Uuid,
        account_id -> Uuid,
        name -> Varchar,
    }
}

table! {
    email_addresses (id) {
        id -> Uuid,
        account -> Varchar,
        domain -> Varchar,
    }
}

table! {
    phone_numbers (id) {
        id -> Uuid,
        phone -> Varchar,
        extension -> Varchar,
    }
}

table! {
    physical_addresses (id) {
        id -> Uuid,
        post_office_box -> Nullable<Varchar>,
        extension -> Nullable<Varchar>,
        street -> Nullable<Varchar>,
        locality -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
    }
}

table! {
    contacts (id) {
        id -> Uuid,
        addressbook_id -> Uuid,
        last_name -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        name_prefix -> Nullable<Varchar>,
        name_suffix -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    organizations (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    organization_contacts (id) {
        id -> Uuid,
        organization_id -> Uuid,
        contact_id -> Uuid,
        created_at -> Uuid,
    }
}

table! {
    organization_phones (id) {
        id -> Uuid,
        organization_id -> Uuid,
        phone_id -> Uuid,
        phone_type -> Varchar,
    }
}

table! {
    organization_emails (id) {
        id -> Uuid,
        organization_id -> Uuid,
        email_id -> Uuid,
        email_type -> Varchar,
    }
}

table! {
    organization_addresses (id) {
        id -> Uuid,
        organization_id -> Uuid,
        address_id -> Uuid,
        address_type -> Varchar,
    }
}

joinable!(organization_phones -> organizations (organization_id));
joinable!(organization_emails -> organizations (organization_id));
joinable!(organization_addresses -> organizations (organization_id));

table! {
    contact_phones (id) {
        id -> Uuid,
        contact_id -> Uuid,
        phone_id -> Uuid,
        phone_type -> Varchar,
    }
}

table! {
    contact_emails (id) {
        id -> Uuid,
        contact_id -> Uuid,
        email_id -> Uuid,
        email_type -> Varchar,
    }
}

table! {
    contact_addresses (id) {
        id -> Uuid,
        contact_id -> Uuid,
        address_id -> Uuid,
        address_type -> Varchar,
    }
}

table! {
    shared_addressbooks (id) {
        id -> Uuid,
        owner_id -> Uuid,
        addressbook_id -> Uuid,
        account_id -> Uuid,
    }
}

table! {
    addressbook_tags (id) {
        id -> Uuid,
        addressbook_id -> Uuid,
        label -> Varchar,
    }
}

table! {
    contact_tags (id) {
        id -> Uuid,
        contact_id -> Uuid,
        tag_id -> Uuid,
    }
}

table! {
    organization_tags (id) {
        id -> Uuid,
        organization_id -> Uuid,
        tag_id -> Uuid,
    }
}

allow_tables_to_appear_in_same_query!(
    contacts, organizations, phone_numbers, email_addresses, physical_addresses,
    contact_emails, contact_phones, contact_addresses, organization_phones, organization_emails,
    organization_addresses
);
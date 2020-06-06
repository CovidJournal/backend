CREATE TABLE "public"."infection"
(
    "id" uuid NOT NULL,
    "organization_id" uuid NOT NULL,
    "places_ids" _uuid NOT NULL,
    "start_timestamp" timestamptz NOT NULL,
    "end_timestamp" timestamptz NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."checkin"
(
    "id" uuid NOT NULL,
    "place_id" uuid NOT NULL,
    "device_id" uuid NOT NULL,
    "user_id" uuid NOT NULL,
    "start_timestamp" timestamptz NOT NULL,
    "end_timestamp" timestamptz NOT NULL,
    "duration" int8 NOT NULL,
    "potential_infection" bool NOT NULL DEFAULT 'FALSE',
    "confirmed" uuid NOT NULL,
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."place"
(
    "id" uuid NOT NULL,
    "organization_id" uuid NOT NULL,
    "name" text NOT NULL,
    "description" text,
    "average_duration" int8 NOT NULL,
    "disabled" bool NOT NULL DEFAULT 'FALSE',
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."organization"
(
    "id" uuid NOT NULL,
    "user_id" uuid NOT NULL,
    "name" text NOT NULL,
    "confirmed" bool NOT NULL,
    "disabled" bool NOT NULL DEFAULT 'FALSE',
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."device"
(
    "id" uuid NOT NULL,
    "user_id" uuid NOT NULL,
    "description" text NOT NULL,
    "hashed_token" text NOT NULL,
    "hashed_confirmation_token" text,
    "confirmed" bool NOT NULL DEFAULT 'FALSE',
    "disabled" bool NOT NULL DEFAULT 'FALSE',
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),

    PRIMARY KEY ("id")
);

CREATE TABLE "public"."user"
(
    "id" uuid NOT NULL,
    "login" text NOT NULL,
    "role" text NOT NULL,
    "confirmed" bool NOT NULL DEFAULT 'FALSE',
    "disabled" bool NOT NULL DEFAULT 'FALSE',
    "updated_at" timestamptz NOT NULL DEFAULT NOW(),
    "created_at" timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);

SELECT diesel_manage_updated_at('infection');
SELECT diesel_manage_updated_at('checkin');
SELECT diesel_manage_updated_at('place');
SELECT diesel_manage_updated_at('organization');
SELECT diesel_manage_updated_at('device');
SELECT diesel_manage_updated_at('user');

ALTER TABLE "public"."organization" ADD FOREIGN KEY ("user_id") REFERENCES "public"."user" ("id") ON DELETE CASCADE;
ALTER TABLE "public"."infection" ADD FOREIGN KEY ("organization_id") REFERENCES "public"."organization" ("id") ON DELETE CASCADE;
ALTER TABLE "public"."checkin" ADD FOREIGN KEY ("place_id") REFERENCES "public"."place" ("id") ON DELETE CASCADE;
ALTER TABLE "public"."checkin" ADD FOREIGN KEY ("device_id") REFERENCES "public"."device" ("id") ON DELETE CASCADE;
ALTER TABLE "public"."checkin" ADD FOREIGN KEY ("user_id") REFERENCES "public"."user" ("id") ON DELETE CASCADE;
ALTER TABLE "public"."place" ADD FOREIGN KEY ("organization_id") REFERENCES "public"."organization" ("id") ON DELETE CASCADE;
ALTER TABLE "public"."device" ADD FOREIGN KEY ("user_id") REFERENCES "public"."user" ("id") ON DELETE CASCADE;
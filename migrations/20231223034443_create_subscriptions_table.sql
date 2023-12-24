create table subscriptions(
   id uuid not null,
   primary key (id),
   email citext not null unique,
   name text not null,
   subscribed_at timestamptz not null
);

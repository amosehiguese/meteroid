-- TODO improve

--: SubscriptionToInvoice(billing_end_date?)
--! subscription_to_invoice_candidates (input_date) : SubscriptionToInvoice
SELECT s.id AS subscription_id,
       s.tenant_id,
       s.customer_id,
       pp.plan_id,
       s.plan_version_id,
       s.billing_start_date,
       s.billing_end_date,
       s.billing_day,
       s.effective_billing_period,
       s.input_parameters,
       pp.currency,
       pp.net_terms,
       pp.version
FROM subscription s
         JOIN plan_version pp ON s.plan_version_id = pp.id
         LEFT JOIN invoice i ON s.id = i.subscription_id AND i.invoice_date > :input_date
where (s.billing_end_date is null OR s.billing_end_date < :input_date)
  AND i.id IS NULL;

--: GetSubscriptionCurrentPeriod(id, billing_start_date, billing_end_date?, billing_day, effective_billing_period, input_parameters, customer_id, customer_external_id?, currency, net_terms)
--! get_subscription_current_period (subscription_id): GetSubscriptionCurrentPeriod
SELECT s.id,
       s.tenant_id,
       s.plan_version_id,
       s.billing_start_date,
       s.billing_end_date,
       s.billing_day,
       s.effective_billing_period,
       s.input_parameters,
       s.customer_id,
       c.alias as customer_external_id,
--        cbp.current_period_start_date,
--        cbp.current_period_end_date,
--        cbp.current_period_idx::integer,
       pp.currency,
       s.net_terms
FROM subscription s
         JOIN plan_version pp ON s.plan_version_id = pp.id
         JOIN customer c ON s.customer_id = c.id
         JOIN current_billing_period cbp ON s.id = cbp.subscription_id
WHERE s.id = :subscription_id;

--! create_subscription (billing_end?, parameters?)
INSERT INTO subscription (id,
                          tenant_id,
                          customer_id,
                          created_by,
                          plan_version_id,
                          status,
                          billing_start_date,
                          billing_end_date,
                          billing_day,
                          effective_billing_period,
                          input_parameters,
                          net_terms)
VALUES (:id,
        :tenant_id,
        :customer_id,
        :created_by,
        :plan_version_id,
        :status,
        :billing_start,
        :billing_end,
        :billing_day,
        :effective_billing_period,
        :parameters,
        :net_terms)
RETURNING id
;

--: Subscription(billing_end_date?)
--: SubscriptionList(billing_end_date?)
--! list_subscriptions_per_plan : SubscriptionList
SELECT s.id             AS subscription_id,
       s.tenant_id,
       s.customer_id,
       s.plan_version_id,
       s.billing_start_date,
       s.billing_end_date,
       s.billing_day,
       s.effective_billing_period,
       s.input_parameters,
       pp.currency,
       s.net_terms,
       pp.version,
       c.name           as customer_name,
       count(*) OVER () AS total_count
FROM subscription s
         JOIN plan_version pp ON s.plan_version_id = pp.id
         JOIN customer c ON s.customer_id = c.id
WHERE pp.plan_id = :plan_id
  AND s.tenant_id = :tenant_id
ORDER BY s.id DESC
LIMIT :limit OFFSET :offset;

--! subscription_by_id   : Subscription
SELECT s.id   AS subscription_id,
       s.tenant_id,
       s.customer_id,
       s.plan_version_id,
       s.billing_start_date,
       s.billing_end_date,
       s.billing_day,
       s.effective_billing_period,
       s.input_parameters,
       pp.currency,
       s.net_terms,
       pp.version,
       c.name as customer_name
FROM subscription s
         JOIN plan_version pp ON s.plan_version_id = pp.id
         JOIN customer c ON s.customer_id = c.id
WHERE s.id = :subscription_id
  AND s.tenant_id = :tenant_id;

--! update_subscription_status()
UPDATE subscription
SET status = :status
WHERE id = :id;

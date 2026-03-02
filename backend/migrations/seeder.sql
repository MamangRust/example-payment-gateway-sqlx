DO $$
BEGIN
    -- Check if data already exists
    IF NOT EXISTS (SELECT 1 FROM users LIMIT 1) THEN
        
        -- ====================================================================
        -- SEED ROLES
        -- ====================================================================
        INSERT INTO roles (role_name) VALUES
        ('ROLE_ADMIN'),
        ('ROLE_USER'),
        ('ROLE_MERCHANT'),
        ('ROLE_PREMIUM')
        ON CONFLICT (role_name) DO NOTHING;

        -- ====================================================================
        -- SEED USERS (50 users)
        -- Password: bcrypt hash of "password123"
        -- ====================================================================
        INSERT INTO users (firstname, lastname, email, password) VALUES
        ('John', 'Doe', 'john.doe@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Jane', 'Smith', 'jane.smith@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Bob', 'Johnson', 'bob.johnson@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Alice', 'Williams', 'alice.williams@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Charlie', 'Brown', 'charlie.brown@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Diana', 'Davis', 'diana.davis@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Eve', 'Miller', 'eve.miller@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Frank', 'Wilson', 'frank.wilson@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Grace', 'Moore', 'grace.moore@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK'),
        ('Henry', 'Taylor', 'henry.taylor@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYqYV5kqIwK')
        ON CONFLICT (email) DO NOTHING;

        -- ====================================================================
        -- SEED USER ROLES (Assign roles to users)
        -- ====================================================================
        INSERT INTO user_roles (user_id, role_id)
        SELECT u.user_id, r.role_id
        FROM users u
        CROSS JOIN roles r
        WHERE u.user_id <= 3 AND r.role_name = 'admin'
        ON CONFLICT DO NOTHING;

        INSERT INTO user_roles (user_id, role_id)
        SELECT u.user_id, r.role_id
        FROM users u
        CROSS JOIN roles r
        WHERE u.user_id > 3 AND r.role_name = 'user'
        ON CONFLICT DO NOTHING;

        -- ====================================================================
        -- SEED CARDS (100 cards, ~10 per user)
        -- ====================================================================
        INSERT INTO cards (user_id, card_number, card_type, expire_date, cvv, card_provider)
        SELECT 
            ((n - 1) % 10) + 1 as user_id,
            LPAD((4000000000000000 + n)::TEXT, 16, '0') as card_number,
            CASE (n % 3)
                WHEN 0 THEN 'debit'
                WHEN 1 THEN 'credit'
                ELSE 'prepaid'
            END as card_type,
            (CURRENT_DATE + ((n % 5) + 1) * INTERVAL '1 year')::DATE as expire_date,
            LPAD((100 + (n % 900))::TEXT, 3, '0') as cvv,
            CASE (n % 4)
                WHEN 0 THEN 'Visa'
                WHEN 1 THEN 'Mastercard'
                WHEN 2 THEN 'American Express'
                ELSE 'Discover'
            END as card_provider
        FROM generate_series(1, 100) as n
        ON CONFLICT (card_number) DO NOTHING;

        -- ====================================================================
        -- SEED SALDOS (Balance for each card)
        -- ====================================================================
        INSERT INTO saldos (card_number, total_balance, withdraw_amount)
        SELECT 
            card_number,
            (1000000 + (card_id * 50000)) as total_balance,
            0 as withdraw_amount
        FROM cards
        ON CONFLICT DO NOTHING;

        -- ====================================================================
        -- SEED MERCHANTS (20 merchants)
        -- ====================================================================
        INSERT INTO merchants (name, api_key, user_id, status)
        SELECT 
            CASE n
                WHEN 1 THEN 'Amazon Store'
                WHEN 2 THEN 'Walmart Supermarket'
                WHEN 3 THEN 'Best Buy Electronics'
                WHEN 4 THEN 'Target Retail'
                WHEN 5 THEN 'Apple Store'
                WHEN 6 THEN 'Starbucks Coffee'
                WHEN 7 THEN 'McDonalds'
                WHEN 8 THEN 'Nike Store'
                WHEN 9 THEN 'Zara Fashion'
                WHEN 10 THEN 'Spotify Premium'
                WHEN 11 THEN 'Netflix Streaming'
                WHEN 12 THEN 'Uber Rides'
                WHEN 13 THEN 'Airbnb Stays'
                WHEN 14 THEN 'Steam Games'
                WHEN 15 THEN 'PlayStation Store'
                WHEN 16 THEN 'Google Play'
                WHEN 17 THEN 'App Store'
                WHEN 18 THEN 'Microsoft Store'
                WHEN 19 THEN 'Adobe Creative'
                ELSE 'Generic Merchant ' || n
            END as name,
            'api_key_' || MD5(random()::TEXT) as api_key,
            ((n - 1) % 10) + 1 as user_id,
            'active' as status
        FROM generate_series(1, 20) as n
        ON CONFLICT (api_key) DO NOTHING;

        -- ====================================================================
        -- SEED TOPUPS (1000 topups - last 30 days)
        -- ====================================================================
        INSERT INTO topups (card_number, topup_amount, topup_method, topup_time, status)
        SELECT 
            c.card_number,
            (50000 + (n % 20) * 25000) as topup_amount,
            CASE (n % 4)
                WHEN 0 THEN 'bank_transfer'
                WHEN 1 THEN 'credit_card'
                WHEN 2 THEN 'e-wallet'
                ELSE 'cash'
            END as topup_method,
            (CURRENT_TIMESTAMP - ((n % 30) || ' days')::INTERVAL - ((n % 24) || ' hours')::INTERVAL) as topup_time,
            CASE WHEN n % 20 = 0 THEN 'pending' ELSE 'completed' END as status
        FROM generate_series(1, 1000) as n
        CROSS JOIN LATERAL (
            SELECT card_number FROM cards ORDER BY RANDOM() LIMIT 1
        ) c
        ON CONFLICT DO NOTHING;

        -- ====================================================================
        -- SEED TRANSACTIONS (5000 transactions - last 60 days)
        -- ====================================================================
        INSERT INTO transactions (card_number, amount, payment_method, merchant_id, transaction_time, status)
        SELECT 
            c.card_number,
            (10000 + (n % 50) * 5000) as amount,
            CASE (n % 5)
                WHEN 0 THEN 'credit_card'
                WHEN 1 THEN 'debit_card'
                WHEN 2 THEN 'e-wallet'
                WHEN 3 THEN 'bank_transfer'
                ELSE 'qr_code'
            END as payment_method,
            ((n % 20) + 1) as merchant_id,
            (CURRENT_TIMESTAMP - ((n % 60) || ' days')::INTERVAL - ((n % 24) || ' hours')::INTERVAL) as transaction_time,
            CASE WHEN n % 100 = 0 THEN 'failed' WHEN n % 50 = 0 THEN 'pending' ELSE 'completed' END as status
        FROM generate_series(1, 5000) as n
        CROSS JOIN LATERAL (
            SELECT card_number FROM cards ORDER BY RANDOM() LIMIT 1
        ) c
        ON CONFLICT DO NOTHING;

        -- ====================================================================
        -- SEED TRANSFERS (500 transfers - last 30 days)
        -- ====================================================================
        INSERT INTO transfers (transfer_from, transfer_to, transfer_amount, transfer_time, status)
        SELECT 
            c1.card_number as transfer_from,
            c2.card_number as transfer_to,
            (20000 + (n % 30) * 10000) as transfer_amount,
            (CURRENT_TIMESTAMP - ((n % 30) || ' days')::INTERVAL - ((n % 24) || ' hours')::INTERVAL) as transfer_time,
            CASE WHEN n % 50 = 0 THEN 'pending' ELSE 'completed' END as status
        FROM generate_series(1, 500) as n
        CROSS JOIN LATERAL (
            SELECT card_number FROM cards ORDER BY RANDOM() LIMIT 1
        ) c1
        CROSS JOIN LATERAL (
            SELECT card_number FROM cards WHERE card_number != c1.card_number ORDER BY RANDOM() LIMIT 1
        ) c2
        ON CONFLICT DO NOTHING;

        -- ====================================================================
        -- SEED WITHDRAWS (800 withdraws - last 30 days)
        -- ====================================================================
        INSERT INTO withdraws (card_number, withdraw_amount, withdraw_time, status)
        SELECT 
            c.card_number,
            (100000 + (n % 25) * 20000) as withdraw_amount,
            (CURRENT_TIMESTAMP - ((n % 30) || ' days')::INTERVAL - ((n % 24) || ' hours')::INTERVAL) as withdraw_time,
            CASE WHEN n % 40 = 0 THEN 'pending' ELSE 'completed' END as status
        FROM generate_series(1, 800) as n
        CROSS JOIN LATERAL (
            SELECT card_number FROM cards ORDER BY RANDOM() LIMIT 1
        ) c
        ON CONFLICT DO NOTHING;

        -- ====================================================================
        -- SUCCESS MESSAGE
        -- ====================================================================
        RAISE NOTICE '✅ Database seeded successfully!';
        RAISE NOTICE '   - 10 users created';
        RAISE NOTICE '   - 100 cards created';
        RAISE NOTICE '   - 20 merchants created';
        RAISE NOTICE '   - 1,000 topups created';
        RAISE NOTICE '   - 5,000 transactions created';
        RAISE NOTICE '   - 500 transfers created';
        RAISE NOTICE '   - 800 withdraws created';
        RAISE NOTICE '';
        RAISE NOTICE '📝 Default credentials:';
        RAISE NOTICE '   Email: john.doe@example.com';
        RAISE NOTICE '   Password: password123';
        
    ELSE
        RAISE NOTICE '⚠️  Data already exists, skipping seed';
    END IF;
END $$;

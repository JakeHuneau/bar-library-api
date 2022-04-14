INSERT INTO ingredients (id, name) 
VALUES
    ('ba2c546b-93af-47a9-8c66-e7f3d38d6889', 'bourbon'),
    ('149d7a96-577c-4993-83e9-3cd5e2173d38', 'gin'),
    ('ae21d318-8e57-4c07-9289-a6ed3cfdf711', 'angostura bitters'),
    ('25f57576-23c2-4a58-a0f3-a2f062531f8a', 'sugar'),
    ('d966f80b-3815-44bf-8cff-890e712d8057', 'water'),
    ('4737a3a0-f4a1-429c-a47d-8e8b6fdedc0e', 'orange'),
    ('ec92a175-c238-4b33-a9ce-b2b690c4317e', 'cherry'),
    ('b0e4f099-bc98-41ee-a5da-75c20a885cb8', 'simple syrup'),
    ('6654c554-73c3-4944-8498-e19bf539854d', 'maraschino liqueur'),
    ('c40c663e-5233-418e-a74e-27249587b60b', 'lemon juice'),
    ('83a051ed-b33c-4367-8f10-75d4c1662e2f', 'egg white'),
    ('3ff37ee3-63a4-4dc3-95ee-0ade0346bad2', 'lemon peel'),
    ('02a8a76b-108a-4445-81c7-219fe6c0fdde', 'brandied cherry'),
    ('a7396779-3654-4491-9ca2-4bfa04683ecf', 'creme de violette'),
    ('b64e1c24-c196-4b62-b7dd-61f49b99d757', 'scotch'),
    ('597bb6fa-705b-4f88-8385-db224e8ab64c', 'amaretto');

INSERT INTO recipes (id, name, directions)
VALUES
    ('ade3c0a4-8e6e-427a-80d3-8e5466c96eb1', 'old fashioned', '1. Saturate sugar cube with bitters and a bit of water. 2. Muddle until dissolved. 3. Fill glass with ice and whiskey. 4. Garnish with orange slice and cherry.'),
    ('d361ff4e-8b51-4db1-91e1-c4341ffc7e4d', 'aviation', '1. shake and strain into chilled cocktail glass.'),
    ('1fec27d7-0b47-4203-8921-41a7fd8ff343', 'godfather', '1. pour all of the ingredients directly into a rocks glass filled with ice cubes. 2. stir gently.'),
    ('6c5fc5a2-a047-462f-b29f-93761a3ffe47', 'amaretto sour', '1. add all ingredients to a cocktail shaker and dry shake. 2. add ice and shake well. 3. strain over ice into a rocks glass. 4. garnish with a lemon peel and a brandied cherry.');

INSERT INTO recipe_ingredients (id, recipe_id, ingredient_id, quantity, unit, required)
VALUES
    ('45592ba8-3255-4772-9f1e-87d85e52d896', 'ade3c0a4-8e6e-427a-80d3-8e5466c96eb1', 'ba2c546b-93af-47a9-8c66-e7f3d38d6889', '1.5', 'oz', '1'),
    ('3db5b077-a453-4029-b9de-4a8667e14b9b', 'ade3c0a4-8e6e-427a-80d3-8e5466c96eb1', '25f57576-23c2-4a58-a0f3-a2f062531f8a', '1.0', 'cube', '1'),
    ('3e709472-1411-454c-ab8c-8f8fd3896ebd', 'ade3c0a4-8e6e-427a-80d3-8e5466c96eb1', 'ae21d318-8e57-4c07-9289-a6ed3cfdf711', '3.0', 'dash', '1'),
    ('aae82300-5437-4c93-9de5-d0f82c243d4a', 'ade3c0a4-8e6e-427a-80d3-8e5466c96eb1', 'd966f80b-3815-44bf-8cff-890e712d8057', '1.0', 'splash', '0'),
    ('4e47ffaa-ce01-4fb3-a6f7-2097bf41176a', 'ade3c0a4-8e6e-427a-80d3-8e5466c96eb1', '4737a3a0-f4a1-429c-a47d-8e8b6fdedc0e', '1.0', 'slice', '1'),
    ('6d190e31-ab82-4d4e-a6d8-bc7714b09da9', 'ade3c0a4-8e6e-427a-80d3-8e5466c96eb1', 'ec92a175-c238-4b33-a9ce-b2b690c4317e', '1.0', '', '0'),
    ('c62f3eca-46f3-48ca-86d0-b2aea9240145', 'd361ff4e-8b51-4db1-91e1-c4341ffc7e4d', '149d7a96-577c-4993-83e9-3cd5e2173d38', '1.5', 'oz', '1'),
    ('100442b3-5191-4134-8b2b-71673f97fa75', 'd361ff4e-8b51-4db1-91e1-c4341ffc7e4d', '6654c554-73c3-4944-8498-e19bf539854d', '0.5', 'oz', '1'),
    ('b5a051b7-7a46-4ca1-b49e-8147f4e830b8', 'd361ff4e-8b51-4db1-91e1-c4341ffc7e4d', 'c40c663e-5233-418e-a74e-27249587b60b', '0.5', 'oz', '1'),
    ('e4a35071-78fe-4fc6-98fb-6abb71159b09', 'd361ff4e-8b51-4db1-91e1-c4341ffc7e4d', 'a7396779-3654-4491-9ca2-4bfa04683ecf', '1.0', 'tsp', '1'),
    ('2eac1378-8a59-4b4a-a284-ba6a2b090095', '1fec27d7-0b47-4203-8921-41a7fd8ff343', 'b64e1c24-c196-4b62-b7dd-61f49b99d757', '1.0', 'oz', '1'),
    ('d6c65e8d-ed72-42f0-8d98-e0464f74e3d5', '1fec27d7-0b47-4203-8921-41a7fd8ff343', '597bb6fa-705b-4f88-8385-db224e8ab64c', '1.0', 'oz', '1'),
    ('98b5f192-d35f-4955-8867-13a9ef65d793', '6c5fc5a2-a047-462f-b29f-93761a3ffe47', '597bb6fa-705b-4f88-8385-db224e8ab64c', '1.5', 'oz', '1'),
    ('470c8390-185f-4a1f-a3b1-a97d1aeac5e9', '6c5fc5a2-a047-462f-b29f-93761a3ffe47', '149d7a96-577c-4993-83e9-3cd5e2173d38', '0.75', 'oz', '1'),
    ('b4d87bf8-9512-44b4-ace6-42cec4a121c5', '6c5fc5a2-a047-462f-b29f-93761a3ffe47', 'c40c663e-5233-418e-a74e-27249587b60b', '1.0', 'oz', '1'),
    ('5399c0da-685a-4364-8aad-532d05d1aedc', '6c5fc5a2-a047-462f-b29f-93761a3ffe47', 'b0e4f099-bc98-41ee-a5da-75c20a885cb8', '1.0', 'tsp', '1'),
    ('a8b91d84-e5bd-436f-8ffa-29f7825d0c6b', '6c5fc5a2-a047-462f-b29f-93761a3ffe47', '83a051ed-b33c-4367-8f10-75d4c1662e2f', '0.5', 'oz', '1'),
    ('49ec2e53-7084-40ba-89b1-43a53c2b1c14', '6c5fc5a2-a047-462f-b29f-93761a3ffe47', '3ff37ee3-63a4-4dc3-95ee-0ade0346bad2', '1.0', '', '0'),
    ('20e4132e-f3bd-49f9-a606-e57b71e94c74', '6c5fc5a2-a047-462f-b29f-93761a3ffe47', '02a8a76b-108a-4445-81c7-219fe6c0fdde', '1.0', '',  '0');

INSERT INTO users (id, name, password, email, permissions, joined_at)
VALUES
    ('be1d71f9-cdcd-4ecd-8fab-e799629c14e2', 'test', '$2a$12$DG/DRiWKje9XiJf5U0PV8.nKw8gFfRvoOzdN5gqgQOyEThxEnDvw2', 'test@test.com', '0', '2016-03-26 10:10:10-05:00'),
    ('578d366c-55a6-4e62-9167-97bf78315ff4', 'superuser', '$2a$12$DG/DRiWKje9XiJf5U0PV8.nKw8gFfRvoOzdN5gqgQOyEThxEnDvw2', 'test2@test.com', '7', '2016-03-26 10:10:10-05:00');

INSERT INTO kitchen (id, user_id, ingredient_id)
VALUES
    ('cd5606dc-255f-47d4-99e9-b08f5c415984', 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2', 'b64e1c24-c196-4b62-b7dd-61f49b99d757'),
    ('9b8cc1c6-325e-4ce0-9051-e4711b6f2733', 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2', '597bb6fa-705b-4f88-8385-db224e8ab64c'),
    ('4f84854a-fa82-4b2a-bb9b-e64ad86ab370', 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2', 'ba2c546b-93af-47a9-8c66-e7f3d38d6889'),
    ('b1ea938a-3710-4434-89a6-4b6f2e5d2922', 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2', '149d7a96-577c-4993-83e9-3cd5e2173d38'),
    ('bb99e194-83a0-411e-9da6-42a00e8d000c', 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2', 'ae21d318-8e57-4c07-9289-a6ed3cfdf711'),
    ('efc4d7f5-c970-4f1b-87ce-226690f197ee', 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2', '25f57576-23c2-4a58-a0f3-a2f062531f8a');

INSERT INTO favorites (id, user_id, recipe_id)
VALUES
    ('4f84854a-fa82-4b2a-bb9b-e64ad86ab334', 'be1d71f9-cdcd-4ecd-8fab-e799629c14e2', 'ade3c0a4-8e6e-427a-80d3-8e5466c96eb1');
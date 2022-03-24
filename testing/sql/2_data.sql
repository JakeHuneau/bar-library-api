COPY ingredients (id, name) FROM stdin;
1	bourbon
2	gin
3	angostura bitters
4	sugar
5	water
6	orange
7	cherry
8	simple syrup
9	maraschino liqueur
10	lemon juice
11	egg white
12	lemon peel
13	brandied cherry
14	creme de violette
15	scotch
16	amaretto
\.

COPY recipes (id, name, directions) FROM stdin;
1	old fashioned	1. Saturate sugar cube with bitters and a bit of water. 2. Muddle until dissolved. 3. Fill glass with ice and whiskey. 4. Garnish with orange slice and cherry.
2	aviation	1. shake and strain into chilled cocktail glass.
3	godfather	1. pour all of the ingredients directly into a rocks glass filled with ice cubes. 2. stir gently.
4	amaretto sour	1. add all ingredients to a cocktail shaker and dry shake. 2. add ice and shake well. 3. strain over ice into a rocks glass. 4. garnish with a lemon peel and a brandied cherry.
\.

COPY recipe_ingredients (id, recipe_id, ingredient_id, quantity, unit, required) FROM stdin;
1	1	1	1.5	oz	1
2	1	4	1.0	cube	1
3	1	3	3.0	dash	1
4	1	5	1.0	splash	0
5	1	6	1.0	slice	1
6	1	7	1.0		0
7	2	2	1.5	oz	1
8	2	9	0.5	oz	1
9	2	10	0.5	oz	1
10	2	14	1.0	tsp	1
11	3	15	1.0	oz	1
12	3	16	1.0	oz	1
13	4	16	1.5	oz	1
14	4	2	0.75	oz	1
15	4	10	1.0	oz	1
16	4	8	1.0	tsp	1
17	4	11	0.5	oz	1
18	4	12	1.0		0
19	4	13	1.0		0
\.

COPY users (id, name, password, permissions) FROM stdin;
1	test	$2b$12$2gBPpFPGTEPPZXRqcq0T9O3YiP0U4hdfsKLVC86yjOTzAvOvdczbW	0
\.

COPY kitchen (id, user_id, ingredient_id) FROM stdin;
1	1	15
2	1	16
3	1	1
4	1	2
5	1	3
6	1	4
\.

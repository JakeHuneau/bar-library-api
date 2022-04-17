ALTER TABLE ONLY recipe_ingredients
ADD CONSTRAINT recipe_ingredients_ibfk_1 foreign key (recipe_id) references recipes (id) ON DELETE CASCADE,
    ADD CONSTRAINT recipe_ingredients_ibfk_2 foreign key (ingredient_id) references ingredients (id) ON DELETE CASCADE;
    
ALTER TABLE ONLY kitchen
ADD CONSTRAINT kitchen_ibfk_1 foreign key (user_id) references users (id) ON DELETE CASCADE,
    ADD CONSTRAINT kitchen_ibfk_2 foreign key (ingredient_id) references ingredients (id) ON DELETE CASCADE;

ALTER TABLE ONLY favorites
ADD CONSTRAINT favorites_ibfk_1 foreign key (user_id) references users (id) ON DELETE CASCADE,
    ADD CONSTRAINT favorites_ibfk_2 foreign key (recipe_id) references recipes (id) ON DELETE CASCADE;
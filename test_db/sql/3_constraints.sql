ALTER TABLE ONLY recipe_ingredients
ADD CONSTRAINT recipe_ingredients_ibfk_1 foreign key (recipe_id) references recipes (id),
    ADD CONSTRAINT recipe_ingredients_ibfk_2 foreign key (ingredient_id) references ingredients (id);
    
ALTER TABLE ONLY kitchen
ADD CONSTRAINT kitchen_ibfk_1 foreign key (user_id) references users (id),
    ADD CONSTRAINT kitchen_ibfk_2 foreign key (ingredient_id) references ingredients (id);
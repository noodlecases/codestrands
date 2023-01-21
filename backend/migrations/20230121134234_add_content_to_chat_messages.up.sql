ALTER TABLE chat_messages
ADD content text NOT NULL DEFAULT '';

ALTER TABLE chat_messages
ALTER COLUMN content 
DROP DEFAULT; 

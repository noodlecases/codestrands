import { useEffect, useState } from "react";
import Navbar from "../components/Navbar";
import useWebSocket from "react-use-websocket";
import { FaceFrownIcon, UserCircleIcon } from "@heroicons/react/24/outline";
import {
  apiGetUserMe,
  Chat,
  getChats,
  getChatUsers,
  getMe,
  getUser,
  getUserChats,
  Message,
  User,
  UserChat,
  UserResponse,
} from "../api";

type MessageProps = {
  message: Message;
  currentUserId: number;
};

const MessageComponent = (props: MessageProps) => {
  let { message, currentUserId } = props;

  const [user, setUser] = useState<UserResponse>();

  useEffect(() => {
    const fetchUser = async () => {
      setUser(await getUser(message.user_id));
    };

    fetchUser();
  }, [message.user_id]);

  if (user === undefined) {
    return null;
  }

  return (
    <div
      className={`chat chat-${
        currentUserId === message.user_id ? "end" : "start"
      }`}
    >
      <div className="chat-image avatar">
        <div className="w-10 rounded-full">
          {!!user.image ? <img src={user.image} /> : <UserCircleIcon />}
        </div>
      </div>
      <div className="chat-header">
        {user.firstName} {user.lastName}
        <time className="ml-1 text-xs opacity-50">
          {message.timestamp.getHours().toString().padStart(2, "0")}:
          {message.timestamp.getMinutes().toString().padStart(2, "0")}
        </time>
      </div>
      <div className="chat-bubble">{message.content}</div>
    </div>
  );
};

const UserComponent = (props: { chatId: number; userId: number }) => {
  const [chatUsers, setChatUsers] = useState<UserChat[]>([]);
  const [users, setUsers] = useState<User[]>([]);

  useEffect(() => {
    const fetchChatUsers = async () => {
      setChatUsers(await getChatUsers(props.chatId));
    };

    fetchChatUsers();
  }, [props.chatId]);

  useEffect(() => {
    const fetchUsers = async () => {
      for (const chatUser of chatUsers) {
        if (chatUser.id === props.userId) continue;

        const user = await getUser(chatUser.userId);
        setUsers((users) => {
          console.log(user)
          if (users.find((u) => u.id === user.id)) {
            return users;
          }
          return users.concat(user);
        });
      }
    };

    fetchUsers();
  }, [chatUsers, props.userId]);

  if (users.length === 0) {
    return null;
  }

  return (
    <div className="flex">
      <div className="chat-image avatar">
        {users
          .filter((user) => user.id !== props.userId)
          .map((user) => (
            <>
              <div className="w-8 h-8 mr-1 rounded-full">
                {!!user.image ? <img src={user.image} /> : <UserCircleIcon />}
              </div>
              <a className="m-1">
                {users
                  .filter((user) => user.id !== props.userId)
                  .map((user) => `${user.firstName} ${user.lastName}`)
                  .join(", ")}
              </a>
            </>
          ))}
      </div>
    </div>
  );
};

const Chat = () => {
  const [socketUrl, setSocketUrl] = useState(
    "ws://codestrands.local:8000/api/v1/chats/ws/"
  );

  const [messages, setMessages] = useState<Message[]>([
    { chat_id: 1, timestamp: new Date(), user_id: 1, content: "hi loser" },
    { chat_id: 1, timestamp: new Date(), user_id: 1, content: "L" },
    {
      chat_id: 1,
      timestamp: new Date(),
      user_id: 1,
      content: "what are you doing with your life",
    },
    { chat_id: 1, timestamp: new Date(), user_id: 2, content: ":(" },
    { chat_id: 1, timestamp: new Date(), user_id: 2, content: "that's mean" },
    { chat_id: 1, timestamp: new Date(), user_id: 1, content: "deal with it" },
    { chat_id: 1, timestamp: new Date(), user_id: 2, content: "D:" },
    { chat_id: 1, timestamp: new Date(), user_id: 3, content: "L" },
  ]);
  const { sendJsonMessage, lastJsonMessage } = useWebSocket(socketUrl);

  const [inputText, setInputText] = useState("");

  const [me, setMe] = useState<User>();
  const [chats, setChats] = useState<Chat[]>([]);

  useEffect(() => {
    const fetchMe = async () => {
      setMe(await getMe());
    };

    fetchMe();
  }, []);

  useEffect(() => {
    const fetchChats = async () => {
      setChats(await getChats());
    };

    fetchChats();
  }, []);

  useEffect(() => {
    if (lastJsonMessage !== null) {
      setMessages((prev) => prev.concat(lastJsonMessage));
    }
  }, [lastJsonMessage]);

  if (me === undefined) {
    return null;
  }

  return (
    <div className="flex justify-center">
      <div>
        <Navbar />
      </div>
      <div className="h-screen w-[60%] border-base-content border-x-2">
        <div className="flex">
          {chats.length > 0 ? (
            <ul className="menu bg-base-300 w-72">
              {chats.map((chat) => (
                <li>
                  <UserComponent chatId={chat.id} userId={me.id} />
                </li>
              ))}
            </ul>
          ) : (
            <div className="flex justify-center items-center w-72">
              <div>
                <p>No matches</p>
                <FaceFrownIcon className="w-16 h-16 mx-auto mt-2" />
              </div>
            </div>
          )}
          <div className="h-screen overflow-y-auto relative w-full border-base-content border-l-2">
            <div className="flex sticky top-0 z-50 justify-center h-12 p-3 bg-base-300">
              Name
            </div>

            <div className="m-4 h-[685px]">
              {messages.map((message) => (
                <MessageComponent message={message} currentUserId={me.id} />
              ))}
            </div>

            <div className="sticky bottom-0 pb-4">
              <div className="flex justify-center items-baseline">
                <div className="form-control">
                  <div className="input-group">
                    <input
                      type="text"
                      placeholder="Send a message"
                      className="input input-bordered w-96"
                      value={inputText}
                      onChange={(event) => setInputText(event.target.value)}
                    />
                    <button className="btn btn-square">
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        strokeWidth={1.5}
                        stroke="currentColor"
                        className="w-6 h-6"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5"
                        />
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Chat;

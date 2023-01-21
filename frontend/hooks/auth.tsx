import {
    ReactNode,
    createContext,
    useContext,
    useEffect,
    useState,
} from "react";

type AuthContextType = {
    isAuthenticated: boolean;
    setIsAuthenticated: (isAuthenticated: boolean) => void;
};

type AuthProviderProps = {
    isAuthenticated?: boolean;
    children: ReactNode;
};

const AuthContext = createContext<AuthContextType>(undefined!);

export const AuthProvider = (props: AuthProviderProps) => {
    const [isAuthenticated, setIsAuthenticated] = useState<boolean>(
        !!props.isAuthenticated
    );

    useEffect(() => {
        localStorage.setItem("isAuthenticated", isAuthenticated.toString());
    }, [isAuthenticated]);

    return (
        <AuthContext.Provider value={{ isAuthenticated, setIsAuthenticated }}>
            {props.children}
        </AuthContext.Provider>
    );
};

export const useAuth = () => useContext(AuthContext);
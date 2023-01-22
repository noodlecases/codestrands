import type {NextPage} from "next";
import Head from "next/head";
import Image from "next/image";

const Home: NextPage = () => {
    return (
        <div className="flex min-h-screen flex-col items-center justify-center py-2">
            <Head>
                <title>CodeStrands</title>
                <link rel="icon" href="/favicon.ico"/>
            </Head>

            <main className="flex w-full flex-1 flex-col items-center justify-center px-20 text-center">
                <Image src="/../public/assets/logo.png" alt="NoodleCases" className="m-8" width={78} height={16}/>
                <h1 className="text-6xl font-bold">
                    Welcome to{" "}
                    <a className="text-transparent bg-clip-text bg-gradient-to-r from-green-300 via-blue-500 to-purple-600">
                        CodeStrands!
                    </a>
                </h1>

                <p className="mt-3 text-2xl">
                    Find others to work with on personal projects!{" "}
                </p>

                <button className="btn btn-info m-8" onClick={(e) => {
                    e.preventDefault();
                    window.location.href = 'https://id.noodlecases.tech/realms/dev/protocol/openid-connect/auth?client_id=codestrands-dev&redirect_uri=http%3A%2F%2Fcodestrands.local%3A8000%2Fauth%2Fcallback%2Fkeycloak&response_type=code&scope=openid%20id';
                }}>Get Started Today
                </button>

                <div className="mt-6 flex max-w-4xl flex-wrap items-center justify-around sm:w-full">

                </div>
            </main>

            <footer className="flex h-24 w-full items-center justify-center border-t">
                <p className="flex items-center justify-center gap-2" >
                    Powered by NoodleCases{" "}
                    <Image src="/../public/assets/noodlecases_logo.png" alt="NoodleCases" width={28} height={16}/>
                </p>
            </footer>
        </div>
    );
};

export default Home;

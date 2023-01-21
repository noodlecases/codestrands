import React from 'react'
import Navbar from '../components/Navbar'

export const profile = () => {
  return (
    <div className = "flex justify-center">
        <div className='flex justify-end w-20 p-2'>
            <button className="btn btn-circle btn-outline">
                <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
        </div>
        <div className = "flex h-screen w-[60%] border-base-content border-x-2"> 
            <div className = "bg-base-100 w-96 h-[95vh] rounded-3xl p-4 m-4">
                <div className='flex h-20 bg-primary rounded-3xl items-center p-2'>
                    <div className = "text-2xl font-bold item-center w-full pl-4 text-primary-content">name</div>
                    <div className="avatar">
                        <div className="w-24 rounded-full">
                            <img src="https://placeimg.com/192/192/people" />
                        </div>
                    </div>
                </div>

                <div className="pl-4 pt-4 text-xl text-primary-content font-semibold">Top Skills</div>
                <div className="flex justify-start py-4">
                    {["test", "test", "test"].map((skill: string) =>
                        <div className="badge badge-primary p-4 m-2">{skill}</div>
                    )}
                </div>

                <div className="px-4">
                    <div className="text-xl text-primary-content font-semibold">Bio</div>
                    <div className="text-primary-content leading-relaxed">
                        test
                    </div>
                </div>
                <div className="p-4">
                    <div className="text-xl text-primary-content font-semibold">Other Skills</div>
                    <div className="flex justify-start py-2">
                        {["test", "test", "test"].map((skill: string) =>
                            <div className="badge badge-outline m-0.5">{skill}</div>
                        )}
                    </div>
                </div>
                <div className="p-4">
                    <div className="text-xl text-primary-content font-semibold">Interests</div>
                    <div className="flex justify-start py-2">
                        {["test", "test", "test"].map((skill: string) =>
                            <div className="badge badge-outline m-0.5">{skill}</div>
                        )}
                    </div>
                </div>
            </div>

            <div className = "bg-base-100 w-96 h-[95vh] rounded-3xl p-4 m-4">
                <div className='h-20 bg-primary flex rounded-3xl items-center p-2'>
                    <div className = "text-2xl font-bold item-center w-full pl-4 text-primary-content">Projects</div>
                </div>

                <div className='h-36 bg-base-200 rounded-3xl p-2 my-4'>
                    <div className='flex'>
                        <div className="font-bold pl-2">Project 1</div>
                        <div className="font-light pl-2">Link to project</div>
                    </div>
                    <div className='pl-2 pt-0.5'>
                        Project description
                    </div>
                </div>
            </div>
        </div>
    </div>
  )
}

export default profile
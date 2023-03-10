import Link from 'next/link';
import React, {useEffect, useState} from 'react'
import {InfinitySpin} from 'react-loader-spinner';
import {
    apiDeleteUserInterestMe,
    apiDeleteUserSkillMe, apiGetInterestAll, apiGetSkillAll,
    apiGetUserInterestMe,
    apiGetUserMe,
    apiGetUserProjectMe,
    apiGetUserSkillMe,
    apiGetUserSocialLinkResponse, InterestResponse,
    ProjectResponse, SkillResponse,
    UserInterestResponse,
    UserSkillResponse,
    UserSocialLinkResponse
} from '../api';
import ProjectsCard from '../components/profile/ProjectsCard';

export const profile = () => {
    type Receipt<T> = { received: boolean, response: T }
    const [userResponse, setUserResponse] = useState({createdAt: 0})
    const [userSkillResponse, setUserSkillResponse] = useState<Receipt<UserSkillResponse[]>>({
        received: false,
        response: []
    })
    const [userInterestResponse, setUserInterestResponse] = useState<Receipt<UserInterestResponse[]>>({
        received: false,
        response: []
    })
    const [userSocialLinkResponse, setUserSocialLinkResponse] = useState<Receipt<UserSocialLinkResponse[]>>({
        received: false,
        response: []
    })
    const [userProjectResponse, setUserProjectResponse] = useState<Receipt<ProjectResponse[]>>({
        received: false,
        response: []
    })
    const [skillResponse, setSkillResponse] = useState<Receipt<SkillResponse[]>>({
        received: false,
        response: []
    })
    const [interestResponse, setInterestResponse] = useState<Receipt<InterestResponse[]>>({
        received: false,
        response: []
    })

    useEffect(() => {
        apiGetUserMe().then((res) => {
            setUserResponse(res)
        })
        apiGetUserSkillMe().then((res) => {
            setUserSkillResponse({received: true, response: res})
        })
        apiGetUserInterestMe().then((res) => {
            setUserInterestResponse({received: true, response: res})
        })
        apiGetUserSocialLinkResponse().then((res) => {
            setUserSocialLinkResponse({received: true, response: res})
        })
        apiGetUserProjectMe().then((res) => {
            setUserProjectResponse({received: true, response: res})
        })
        apiGetSkillAll().then((res) => {
            setSkillResponse({received: true, response: res})
        })
        apiGetInterestAll().then((res) => {
            setInterestResponse({received: true, response: res})
        })
    }, [])

    return (
        <div className="flex justify-center">
            <div className='flex justify-end w-20 p-2 m-4'>
                <Link className="btn btn-circle btn-outline" href="/home">
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24"
                         stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"/>
                    </svg>
                </Link>
            </div>
            <div className="flex h-screen w-[50%] border-base-content border-x-2">
                <div className="bg-base-100 w-96 h-[95vh] rounded-3xl p-4 m-4">
                    <div className='flex h-20 bg-primary rounded-3xl items-center p-2'>
                        <div className="text-2xl font-bold item-center w-full pl-4 text-primary-content">name</div>
                        <div className="avatar">
                            <div className="w-24 rounded-full">
                                <img src="https://placeimg.com/192/192/people"/>
                            </div>
                        </div>
                    </div>

                    <div className="pl-4 pt-4 text-xl text-primary-content font-semibold">Top Skills</div>
                    <div className="justify-start py-4">
                        {userSkillResponse.received && skillResponse.received ?
                            userSkillResponse.response.map((x) => {
                                for (let i = 0; i < skillResponse.response.length; i++) {
                                    if (x.skillId === skillResponse.response[i].id) {
                                        return <div
                                            className="badge badge-primary p-4 mb-2 mr-1">{skillResponse.response[i].name}</div>
                                    }
                                }
                            })
                            : <InfinitySpin width='200' color="#4fa94d"/>}
                    </div>

                    <div className="px-4">
                        <div className="text-xl text-primary-content font-semibold">Bio</div>
                        <div className="text-primary-content leading-relaxed">
                            {//@ts-ignore
                                userResponse.bio}
                        </div>
                    </div>
                    <div className="p-4">
                        <div className="text-xl text-primary-content font-semibold">Other Skills</div>
                        <div className="justify-start py-2">
                            {userSkillResponse.received && skillResponse.received ?
                                userSkillResponse.response.map((x) => {
                                    for (let i = 0; i < skillResponse.response.length; i++) {
                                        if (x.skillId === skillResponse.response[i].id) {
                                            return <div
                                                className="badge badge-primary p-4 mb-2 mr-1">{skillResponse.response[i].name}</div>
                                        }
                                    }
                                }) : <InfinitySpin width='200' color="#4fa94d"/>}
                        </div>
                    </div>
                    <div className="px-4">
                        <div className="text-xl text-primary-content font-semibold">Interests</div>
                        <div className="justify-start py-2">
                            {userInterestResponse.received && interestResponse.received ?
                                userInterestResponse.response.map((x) => {
                                    for (let i = 0; i < interestResponse.response.length; i++) {
                                        if (x.interestId === interestResponse.response[i].id) {
                                            return <div
                                                className="badge badge-primary p-4 mb-2 mr-1">{interestResponse.response[i].name}</div>
                                        }
                                    }
                                }) : <InfinitySpin width='200' color="#4fa94d"/>}
                        </div>
                    </div>
                    <div className="p-4">
                        <div className="text-xl text-primary-content font-semibold">My Links</div>
                        <div className="justify-start ml-2 p-2">
                            {userSocialLinkResponse.received ?
                                userSocialLinkResponse.response.map((x) => <li><a href={x.url}
                                                                                  className='text-sm underline'>{x.name}</a>
                                </li>)
                                : <InfinitySpin width='200' color="#4fa94d"/>}
                        </div>
                    </div>
                </div>


                <div className="bg-base-100 w-96 h-[95vh] rounded-3xl p-4 m-4">
                    <div className='h-20 bg-primary flex rounded-3xl items-center p-2'>
                        <div className="text-2xl font-bold item-center w-full pl-4 text-primary-content">Projects</div>
                    </div>

                    {userProjectResponse.received ?
                        userProjectResponse.response.map((x) => <ProjectsCard projectName={x.name}
                                                                              projectDesc={x.description}
                                                                              link={x.url}/>)
                        : <InfinitySpin width='200' color="#4fa94d"/>}
                </div>
            </div>
            <div>

                <Link href="/editprofile" className="btn btn-outline m-6">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5"
                         stroke="currentColor" className="w-6 h-6">
                        <path strokeLinecap="round" strokeLinejoin="round"
                              d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10"/>
                    </svg>
                    Edit Profile
                </Link>
            </div>
        </div>
    )
}

export default profile
import Link from 'next/link';
import React, { useEffect, useState } from 'react'
import { InfinitySpin } from 'react-loader-spinner';
import { apiGetUserInterestMe, apiGetUserMe, apiGetUserProjectMe, apiGetUserSkillMe, apiGetUserSocialLinkResponse, ProjectResponse, UserInterestResponse, UserSkillResponse, UserSocialLinkResponse } from '../api';
import BadgeListForm from '../components/forms/BadgeListForm';
import Navbar from '../components/Navbar'
import ProfileExpanded from '../components/profile/ProfileExpanded'
import ProjectsCard from '../components/profile/ProjectsCard';

const user = {
    username: "Aaron", 
    bio: "test", 
    topSkills: ["skill1", "skill2", "skill3"], 
    skills: ["skill", "skill", "skill", "skill"], 
    interests: ["interests", "interests", "interests", "interests"]
};

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
    }, [])
    
  return (
    <div className = "flex justify-center">
        <div className='flex justify-end w-20 p-2'>
            <Link className="btn btn-circle btn-outline" href="/swipe">
                <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </Link>
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
                    {userSkillResponse.received ?
                        userSkillResponse.response.map((x) => <div className="badge badge-primary p-4 m-2">{x.skillId.toString()}</div>)
                        : <InfinitySpin width='200' color="#4fa94d"/>}
                </div>

                <div className="px-4">
                    <div className="text-xl text-primary-content font-semibold">Bio</div>
                    <div className="text-primary-content leading-relaxed">
                        {userResponse.bio}
                    </div>
                </div>
                <div className="p-4">
                    <div className="text-xl text-primary-content font-semibold">Other Skills</div>
                    <div className="flex justify-start py-2">
                        {userSkillResponse.received ?
                            userSkillResponse.response.map((x) => <div className="badge badge-primary p-4 m-2">{x.skillId.toString()}</div>)
                            : <InfinitySpin width='200' color="#4fa94d"/>}
                    </div>
                </div>
                <div className="px-4">
                    <div className="text-xl text-primary-content font-semibold">Interests</div>
                    <div className="flex justify-start py-2">
                        {userInterestResponse.received ?
                            userInterestResponse.response.map((x) => <div className="badge badge-primary p-4 m-2">{x.interestId.toString()}</div>)
                            : <InfinitySpin width='200' color="#4fa94d"/>}
                    </div>
                </div>
                <div className="p-4">
                    <div className="text-xl text-primary-content font-semibold">My Links</div>
                    <div className="justify-start ml-2 p-2">
                        {userSocialLinkResponse.received ?
                            userSocialLinkResponse.response.map((x) => <li><a href={x.url} className='text-sm underline'>{x.name}</a></li>)
                            : <InfinitySpin width='200' color="#4fa94d"/>}
                    </div>
                </div>
            </div>


            <div className = "bg-base-100 w-96 h-[95vh] rounded-3xl p-4 m-4">
                <div className='h-20 bg-primary flex rounded-3xl items-center p-2'>
                    <div className = "text-2xl font-bold item-center w-full pl-4 text-primary-content">Projects</div>
                </div>
                
                {userProjectResponse.received ?
                    userProjectResponse.response.map((x) => <ProjectsCard projectName={x.name} projectDesc={x.description} link={x.url} />)
                    : <InfinitySpin width='200' color="#4fa94d"/>}
            </div>
        </div>
    </div>
  )
}

export default profile
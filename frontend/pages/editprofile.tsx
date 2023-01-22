import React, {useEffect, useState} from 'react'
import Navbar from '../components/Navbar'
import ProjectCard from '../components/ProjectCard'
import TextFieldForm from "../components/forms/TextFieldForm";
import TextAreaForm from "../components/forms/TextAreaForm";
import BadgeForm from "../components/forms/BadgeForm";
import BadgeListForm from "../components/forms/BadgeListForm";
import {
    UserResponse,
    apiGetUserMe,
    apiGetUserSkillMe,
    apiGetUserInterestMe,
    apiGetUserProjectMe,
    UserSkillResponse, UserInterestResponse, ProjectResponse, apiPatchUserMe
} from "../api"
import {InfinitySpin} from "react-loader-spinner";
import {list} from "postcss";

type UserProp = {
    username: string,
    bio: string,
    topSkills: Array<string>,
    interests: Array<string>,
    skills: Array<string>
}

const editProfile = (props: UserProp) => {
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
        apiGetUserProjectMe().then((res) => {
            setUserProjectResponse({received: true, response: res})
        })
    }, [])

    return (
        <div className="flex justify-center">
            <div>
                <Navbar/>
            </div>
            <div className="h-full w-[50%] border-base-content border-x-2">
                <div className="text-3xl p-8 text-primary-content font-semibold">Edit Profile</div>
                {userResponse.createdAt > 0 ? <div className='px-8 pb-8'>
                    <TextFieldForm fieldName={"First name"} callFunction={(d: string) => {apiPatchUserMe({
                        firstName: d,
                    })}}
                                   placeholder={userResponse.firstName}></TextFieldForm>
                    <TextFieldForm fieldName={"Last name"} callFunction={(d: string) => {apiPatchUserMe({
                        lastName: d,
                    })}}
                                   placeholder={userResponse.lastName}></TextFieldForm>
                    <TextAreaForm fieldName={"Bio"} callFunction={(d: string) => {apiPatchUserMe({
                        bio: d,
                    })}}
                                  placeholder={userResponse.bio}></TextAreaForm>
                </div> : <InfinitySpin width='200' color="#4fa94d"/>}
                <div className='px-8 pb-8'>
                    {userSkillResponse.received ?
                        <BadgeListForm name="Skills" badges={userSkillResponse.response.map((x) => x.skillId.toString())}
                                       buttonCaption="Add new skills"></BadgeListForm>
                        : <InfinitySpin width='200' color="#4fa94d"/>}
                    {userInterestResponse.received ?
                        <BadgeListForm name="Interests" badges={userInterestResponse.response.map((x) => x.interestId.toString())}
                                       buttonCaption="Add new interests"></BadgeListForm>
                        : <InfinitySpin width='200' color="#4fa94d"/>}
                </div>

                <div className='px-8 pb-8'>
                    <div className="text-xl text-primary-content font-semibold pb-4">Remove Projects</div>
                    <ProjectCard/>
                    <ProjectCard/>
                    <ProjectCard/>
                </div>

                <div className='px-8 pb-8'>
                    <div className="text-xl text-primary-content font-semibold pb-4">Add New Project</div>

                    <div className="text-xl text-primary-content font-semibold">Project Name</div>
                    <input type="text" placeholder="Project Name"
                           className="input input-bordered w-full max-w-xs mr-2 mb-2"/>

                    <div className="text-xl text-primary-content font-semibold">Description</div>
                    <textarea rows={5} cols={40} className="textarea textarea-bordered"
                              placeholder="Description"></textarea>

                    <div className="text-xl text-primary-content font-semibold">Link</div>
                    <input type="text" placeholder="Project Link"
                           className="input input-bordered w-full max-w-xs mr-2 mb-2"/>

                    <div className="text-xl text-primary-content font-semibold">Image</div>
                    <input type="file" className="file-input file-input-bordered w-full max-w-xs"/>
                </div>

                <button className="btn btn-primary mx-8 mb-20">Add Project</button>
            </div>
        </div>
    )
}

export default editProfile
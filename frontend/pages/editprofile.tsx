import React, {useEffect, useState} from 'react'
import Navbar from '../components/Navbar'
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
    UserSkillResponse,
    UserInterestResponse,
    ProjectResponse,
    apiPatchUserMe,
    SkillResponse,
    InterestResponse,
    apiGetSkillAll,
    apiGetInterestAll,
    apiPutUserSkillMe,
    apiDeleteUserSkillMe,
    apiDeleteUserInterestMe,
    apiPutUserInterestMe, apiDeleteProjectMe
} from "../api"
import {InfinitySpin} from "react-loader-spinner";
import {list} from "postcss";
import NewProjectForm from "../components/forms/NewProjectForm";
import ManageProjectForm from "../components/forms/ManageProjectForm";

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
            <div>
                <Navbar/>
            </div>
            <div className="h-full w-[50%] border-base-content border-x-2">
                <div className="text-3xl p-8 text-primary-content font-semibold">Edit Profile</div>
                {userResponse.createdAt > 0 ? <div className='px-8 pb-8'>
                    <TextFieldForm fieldName={"First name"} callFunction={(d: string) => {
                        apiPatchUserMe({
                            firstName: d,
                        })
                    }}
                                   placeholder={userResponse.firstName}></TextFieldForm>
                    <TextFieldForm fieldName={"Last name"} callFunction={(d: string) => {
                        apiPatchUserMe({
                            lastName: d,
                        })
                    }}
                                   placeholder={userResponse.lastName}></TextFieldForm>
                    <TextAreaForm fieldName={"Bio"} callFunction={(d: string) => {
                        apiPatchUserMe({
                            bio: d,
                        })
                    }}
                                  placeholder={userResponse.bio}></TextAreaForm>
                </div> : <InfinitySpin width='200' color="#4fa94d"/>}
                <div className='px-8 pb-8'>
                    {userSkillResponse.received && skillResponse.received ?
                        <BadgeListForm name="Skills" badges={userSkillResponse.response.map((x) => {
                            for (let i = 0; i < skillResponse.response.length; i++) {
                                if (x.skillId === skillResponse.response[i].id) {
                                    return {
                                        id: skillResponse.response[i].id,
                                        name: skillResponse.response[i].name,
                                        toggleFunction: apiDeleteUserSkillMe,
                                    }
                                }
                            }
                        })} allBadges={skillResponse.response.map((x) => {
                            return {id: x.id, name: x.name, toggleFunction: apiPutUserSkillMe}
                        })} buttonCaption="Add new skills"></BadgeListForm>
                        : <InfinitySpin width='200' color="#4fa94d"/>}
                    {userInterestResponse.received && interestResponse.received ?
                        <BadgeListForm name="Interests" badges={userInterestResponse.response.map((x) => {
                            for (let i = 0; i < interestResponse.response.length; i++) {
                                if (x.interestId === interestResponse.response[i].id) {
                                    return {
                                        id: interestResponse.response[i].id,
                                        name: interestResponse.response[i].name,
                                        toggleFunction: apiDeleteUserInterestMe,
                                    }
                                }
                            }
                        })} allBadges={interestResponse.response.map((x) => {
                            return {id: x.id, name: x.name, toggleFunction: apiPutUserInterestMe}
                        })} buttonCaption="Add new interests"></BadgeListForm>
                        : <InfinitySpin width='200' color="#4fa94d"/>}
                </div>

                <NewProjectForm></NewProjectForm>

                <div className='px-8 pb-8'>
                    <div className="text-xl text-primary-content font-semibold pb-4">Manage Projects</div>
                    {userProjectResponse.received ? userProjectResponse.response.length === 0 ?
                        <p>You currently do not have any projects. Why not create one?</p>
                        : userProjectResponse.response.map((project) =>
                            <ManageProjectForm name={project.name} url={project.url} projectId={project.id}
                                               description={project.description} image={project.image} key={project.id}
                                               deleteFunction={apiDeleteProjectMe}></ManageProjectForm>
                        ) : <InfinitySpin width='200' color="#4fa94d"/>}
                </div>
            </div>
        </div>
    )
}

export default editProfile
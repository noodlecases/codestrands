import Link from 'next/link'
import React from 'react'

type UserProp = {
    username: string, 
    bio: string, 
    topSkills: Array<string>, 
    skills: Array<string>,
    interests: Array<string>
}

const ProfileExpanded = (props: UserProp) => {
  return (
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
            {props.topSkills.map((skill: string) =>
                <div className="badge badge-primary p-4 m-2">{skill}</div>
            )}
        </div>

        <div className="px-4">
            <div className="text-xl text-primary-content font-semibold">Bio</div>
            <div className="text-primary-content leading-relaxed">
                {props.bio}
            </div>
        </div>
        <div className="p-4">
            <div className="text-xl text-primary-content font-semibold">Other Skills</div>
            <div className="flex justify-start py-2">
                {props.skills.map((skill: string) =>
                    <div className="badge badge-outline m-0.5">{skill}</div>
                )}
            </div>
        </div>
        <div className="px-4">
            <div className="text-xl text-primary-content font-semibold">Interests</div>
            <div className="flex justify-start py-2">
                {props.interests.map((skill: string) =>
                    <div className="badge badge-outline m-0.5">{skill}</div>
                )}
            </div>
        </div>
        <div className="p-4">
            <div className="text-xl text-primary-content font-semibold">My Links</div>
            <div className="justify-start ml-2 p-2">
                {props.interests.map((skill: string) =>
                    <li><Link href="https://www.google.com/" className='text-sm underline'>Stuff</Link> </li>
                )}
            </div>
        </div>
    </div>
  )
}

export default ProfileExpanded
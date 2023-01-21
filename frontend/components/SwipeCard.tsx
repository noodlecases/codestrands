import React from 'react'

const SwipeCard = (user: {username: string, bio: string, topSkills: Array<string>, skills: Array<string>}) => {
  return (
    <div className = "bg-base-100 w-[550px] h-[75vh] rounded-3xl p-4 m-8">
        <div className='h-20 bg-primary flex rounded-3xl items-center p-2'>
            <div className = "text-2xl font-bold item-center w-full pl-4 text-primary-content">{user.username}</div>
            <div className="avatar">
                <div className="w-24 rounded-full">
                    <img src="https://placeimg.com/192/192/people" />
                </div>
            </div>
        </div>

        <div className="flex justify-start py-4">
            {user.topSkills.map((skill: string) =>
                <div className="badge badge-primary p-4 m-2">{skill}</div>
            )}
        </div>

        <div className="px-4">
            <div className="text-xl text-primary-content font-semibold">Bio</div>
            <div className="text-primary-content leading-relaxed">
                {user.bio}
            </div>
        </div>
        <div className="p-4">
            <div className="text-xl text-primary-content font-semibold">Other Skills</div>
            <div className="flex justify-start py-2">
                {user.skills.map((skill: string) =>
                    <div className="badge badge-outline m-0.5">{skill}</div>
                )}
            </div>
        </div>
    </div>
  )
}

export default SwipeCard
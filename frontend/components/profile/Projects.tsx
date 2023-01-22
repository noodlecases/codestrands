import React from 'react'

type UserProp = {
    username: string, 
    bio: string, 
    topSkills: Array<string>, 
    skills: Array<string>,
    interests: Array<string>
}

const Projects = (props: UserProp) => {
  return (
    <div className = "bg-base-100 w-96 h-[95vh] rounded-3xl p-4 m-4">
        <div className='h-20 bg-primary flex rounded-3xl items-center p-2'>
            <div className = "text-2xl font-bold item-center w-full pl-4 text-primary-content">Projects</div>
        </div>

        <div className="card card-side h-36 bg-base-100 my-4 shadow-xl">
            <figure><img className='w-20 rounded-lg' src="https://placeimg.com/200/280/arch" alt="Movie"/></figure>
            <div className="m-2">
                <div className='flow-root'>
                    <div className="text-lg card-title float-left">Project</div>
                </div>
                <div className='text-xs'>Project desc</div>
                <div className="card-actions justify-end">
                </div>
            </div>
        </div>
    </div>
  )
}

export default Projects

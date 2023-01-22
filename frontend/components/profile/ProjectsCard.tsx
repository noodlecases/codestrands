import React from 'react'

type UserProp = {
    projectName: string, 
    projectDesc: string, 
    link: string
}

const ProjectsCard = (props: UserProp) => {
  return (
    <div className="card card-side h-36 bg-base-100 my-4 shadow-xl">
        <figure><img className='w-20 rounded-lg' src="https://placeimg.com/200/280/arch" alt="Movie"/></figure>
        <div className="m-2">
            <div className='flow-root'>
                <div className="text-lg card-title float-left">{props.projectName}</div>
            </div>
            <div className='text-xs'>{props.projectDesc}</div>
            <div className="card-actions justify-end">
            </div>
        </div>
    </div>
  )
}

export default ProjectsCard

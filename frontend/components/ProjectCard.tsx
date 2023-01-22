import React from 'react'

const ProjectCard = () => {
  return (
    <div className="card card-side h-48 bg-base-100 mb-2 shadow-xl">
        <figure><img src="https://placeimg.com/200/280/arch" alt="Movie"/></figure>
        <div className="card-body">
            <div className='flow-root'>
                <h2 className="card-title float-left">Project</h2>
                <a className='justify-end float-right'>Link</a>
            </div>
            <p>Project desc</p>
            <div className="card-actions justify-end">
            <button className="btn btn-circle">
                <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
            </div>
        </div>
    </div>
  )
}

export default ProjectCard

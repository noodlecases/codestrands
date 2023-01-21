import React from 'react'
import Navbar from '../components/Navbar'
import ProjectCard from '../components/ProjectCard'

type UserProp = {
    username: string, 
    bio: string, 
    topSkills: Array<string>,
    interests: Array<string>,
    skills: Array<string>
}

const editProfile = (props: UserProp) => {
  return (
    <div className = "flex justify-center">
        <div>
          <Navbar />
        </div>
        <div className = "h-full w-[50%] border-base-content border-x-2">
            <div className="text-3xl p-8 text-primary-content font-semibold">Edit Profile</div>
            <div className='px-8 pb-8'>
                <div className="text-xl text-primary-content font-semibold">First Name</div>
                <div className='flex pt-2'>
                    <input type="text" placeholder="New first name" className="input input-bordered w-full max-w-xs mr-2" />
                    <button className="btn btn-primary">Update</button>
                </div>
            </div>
            <div className='px-8 pb-8'>
                <div className="text-xl text-primary-content font-semibold">Last Name</div>
                <div className='flex pt-2'>
                    <input type="text" placeholder="New last name" className="input input-bordered w-full max-w-xs mr-2" />
                    <button className="btn btn-primary">Update</button>
                </div>
            </div>
            <div className='px-8 pb-8'>
                <div className="text-xl text-primary-content font-semibold">Bio</div>
                <div className='pt-2'>
                    <textarea rows={5} cols={40} className="textarea textarea-bordered" placeholder="Bio"></textarea>
                </div>
                <button className="btn btn-primary mt-2">Update</button>
            </div>

            <div className='px-8 pb-2'>
                <div className="text-xl text-primary-content font-semibold pb-2">Skills</div>
                
                {/* {props.skills.map((skill: string) =>
                    <div className="badge badge-info gap-2 mr-1">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                        {skill}
                    </div>
                )} */}
                
                <div className="badge badge-info gap-2 mr-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    info
                </div>
                <div className="badge badge-success gap-2 mr-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    success
                </div>
                <div className="badge badge-warning gap-2 mr-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    warning
                </div>
                <div className="badge badge-error gap-2 mr-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    error
                </div>
            </div>
            <div className='px-8 pb-8'>
                <div className="dropdown">
                    <label tabIndex={0} className="btn btn-primary mr-1">Add new skills</label>
                    <ul tabIndex={0} className="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
                        <li><a>Item 1</a></li>
                        <li><a>Item 2</a></li>
                    </ul>
                </div>
            </div>

            <div className='px-8 pb-2'>
                <div className="text-xl text-primary-content font-semibold pb-2">Interests</div>
                
                {/* {props.interests.map((interest: string) =>
                    <div className="badge badge-info gap-2 mr-1">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                        {interest}
                    </div>
                )} */}

                <div className="badge badge-success gap-2 mr-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    success
                </div>
                <div className="badge badge-warning gap-2 mr-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    warning
                </div>
                <div className="badge badge-error gap-2 mr-1">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-4 h-4 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    error
                </div>
            </div>
            <div className='px-8 pb-8'>
                <div className="dropdown">
                    <label tabIndex={0} className="btn btn-primary mr-1">Add new interests</label>
                    <ul tabIndex={0} className="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
                        <li><a>Item 1</a></li>
                        <li><a>Item 2</a></li>
                    </ul>
                </div>
            </div>

            <div className='px-8 pb-8'>
                <div className="text-xl text-primary-content font-semibold pb-4">Remove Projects</div>
                <ProjectCard />
                <ProjectCard />
                <ProjectCard />
            </div>

            <div className='px-8 pb-8'>
                <div className="text-xl text-primary-content font-semibold pb-4">Add New Project </div>
                
                <div className="text-xl text-primary-content font-semibold">Project Name</div>
                <input type="text" placeholder="Project Name" className="input input-bordered w-full max-w-xs mr-2 mb-2" />
                
                <div className="text-xl text-primary-content font-semibold">Description</div>
                <textarea rows={5} cols={40} className="textarea textarea-bordered" placeholder="Description"></textarea>

                <div className="text-xl text-primary-content font-semibold">Link</div>
                <input type="text" placeholder="Project Link" className="input input-bordered w-full max-w-xs mr-2 mb-2" />

                <div className="text-xl text-primary-content font-semibold">Image</div>
                <input type="file" className="file-input file-input-bordered w-full max-w-xs" />
            </div>

            <button className="btn btn-primary mx-8 mb-20">Add Project</button>
        </div>
    </div>
  )
}

export default editProfile
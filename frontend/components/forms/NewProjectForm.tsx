import React from 'react'
import {useRouter} from "next/router";


type UserProp = {
}


const NewProjectForm = (props: UserProp) => {
    const router = useRouter()

    return (
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
    )
}

export default NewProjectForm
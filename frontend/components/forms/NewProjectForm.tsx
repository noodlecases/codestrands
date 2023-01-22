import React, {useState} from 'react'
import {useRouter} from "next/router";
// import aws, {S3} from 'aws-sdk';
import {apiPostProjectMe} from "../../api";

type UserProp = {}

const NewProjectForm = (props: UserProp) => {
    const router = useRouter()

    const [projectName, setProjectName] = useState("")
    const [projectDescription, setProjectDescription] = useState("")
    const [projectLink, setProjectLink] = useState("")
    // const [projectImage, setProjectImage] = useState(undefined)

    const handleProjectNameInput = event => {
        setProjectName(event.target.value)
    }
    const handleProjectDescriptionInput = event => {
        setProjectDescription(event.target.value)
    }
    const handleProjectLinkInput = event => {
        setProjectLink(event.target.value)
    }
    // const handleProjectImageInput = event => {
    //     setProjectImage(event.target.files[0])
    // }

    const handleSubmit = (event) => {
        event.preventDefault()
        apiPostProjectMe({
            name: projectName,
            description: projectDescription,
            url: projectLink,
        }).then(() => {
            router.reload()
        })
    }


    return (
        <div className='px-8 pb-8'>
            <div className="text-xl text-primary-content font-semibold pb-4">Add New Project</div>

            <div className="text-xl text-primary-content font-semibold">Project Name</div>
            <input type="text" placeholder="Project Name" onInput={handleProjectNameInput}
                   className="input input-bordered w-full max-w-xs mr-2 mb-2"/>

            <div className="text-xl text-primary-content font-semibold">Description</div>
            <textarea rows={5} cols={40} className="textarea textarea-bordered" onInput={handleProjectDescriptionInput}
                      placeholder="Description"></textarea>

            <div className="text-xl text-primary-content font-semibold">Link</div>
            <input type="text" placeholder="Project Link" onInput={handleProjectLinkInput}
                   className="input input-bordered w-full max-w-xs mr-2 mb-2"/>

            <div className="text-xl text-primary-content font-semibold">Image</div>
            {/*<form id="image-upload">*/}
            {/*    <input type="file" onChange={handleProjectImageInput}*/}
            {/*           className="file-input file-input-bordered w-full max-w-xs"/>*/}
            {/*</form>*/}
            <button className="btn btn-primary my-5" onClick={handleSubmit}>Add Project</button>
        </div>
    )


}

export default NewProjectForm
import React from 'react'
import {useRouter} from "next/router";

type UserProp = {
    projectId: number
    name: string,
    description: string,
    image?: string,
    url: string,
    deleteFunction: Function,
}


const ManageProjectForm = (props: UserProp) => {
    const router = useRouter()

    return (
        <div className="card card-side h-48 bg-base-100 mb-2 shadow-xl">
            {props.image !== undefined ? <figure><img src={props.image}/></figure> : <p></p>}
            <div className="card-body">
                <div className='flow-root'>
                    <h2 className="card-title float-left">{props.name}</h2>
                    <a className='justify-end float-right' href={props.url}>Link</a>
                </div>
                <p>{props.description}</p>
                <div className="card-actions justify-end">
                    <button className="btn btn-circle" onClick={() => {
                        props.deleteFunction(props.projectId)
                        router.reload()
                    }}>
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24"
                             stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                                  d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    )
}

export default ManageProjectForm

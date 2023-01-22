import React from 'react'
import Navbar from '../components/Navbar'

const chat = () => {
  return (
    <div className = "flex justify-center">
        <div>
          <Navbar />
        </div>
        <div className = "h-screen w-[60%] border-base-content border-x-2">
            <div className='flex'>
                <ul className="menu bg-base-300 w-72">
                    <li>
                        <div className='flex'>
                            <div className="chat-image avatar">
                                <div className="w-8 h-8 mr-1 rounded-full">
                                    <img src="https://placeimg.com/192/192/people" />
                                </div>
                                <a className='m-1'>Item 1</a>
                            </div>
                        </div>
                    </li>
                    <li>
                        <div className='flex'>
                            <div className="chat-image avatar">
                                <div className="w-8 h-8 mr-1 rounded-full">
                                    <img src="https://placeimg.com/192/192/people" />
                                </div>
                                <a className='m-1'>Item 2</a>
                            </div>
                        </div>
                    </li>
                    <li>
                        <div className='flex'>
                            <div className="chat-image avatar">
                                <div className="w-8 h-8 mr-1 rounded-full">
                                    <img src="https://placeimg.com/192/192/people" />
                                </div>
                                <a className='m-1'>Item 3</a>
                            </div>
                        </div>
                    </li>
                </ul>
                <div className='h-screen overflow-y-auto relative w-full border-base-content border-l-2'>
                    <div className='flex sticky top-0 z-50 justify-center h-12 p-3 bg-base-300'>
                        Name
                    </div>
                    <div className='m-4 h-[685px]'>
                        <div className="chat chat-start">
                            <div className="chat-image avatar">
                                <div className="w-10 rounded-full">
                                <img src="https://placeimg.com/192/192/people" />
                                </div>
                            </div>
                            <div className="chat-header">
                                Obi-Wan Kenobi
                                <time className="text-xs opacity-50">12:45</time>
                            </div>
                            <div className="chat-bubble">You were the Chosen One!</div>
                        </div>
                        <div className="chat chat-end">
                            <div className="chat-image avatar">
                                <div className="w-10 rounded-full">
                                <img src="https://placeimg.com/192/192/people" />
                                </div>
                            </div>
                            <div className="chat-header">
                                Anakin
                                <time className="text-xs opacity-50">12:46</time>
                            </div>
                            <div className="chat-bubble">I hate you!</div>
                        </div>
                    </div>
                    
                    <div className='sticky bottom-0 pb-4'>
                        <div className='flex justify-center items-baseline'>
                            <div className="form-control">
                                <div className="input-group">
                                    <input type="text" placeholder="Send a message" className="input input-bordered w-96" />
                                    <button className="btn btn-square">
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                                            <path strokeLinecap="round" strokeLinejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
  )
}

export default chat
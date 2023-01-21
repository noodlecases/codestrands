import React from 'react'
import Navbar from '../components/Navbar'
import SwipeCard from '../components/SwipeCard'

const swipe = () => {
  return (
    <div className = "flex justify-center">
        <div>
          <Navbar />
        </div>
        <div className = "h-screen w-[50%] border-base-content border-x-2">
          <div className="stack animate-fade-in-down">
            <SwipeCard />
            <SwipeCard />
            <SwipeCard />
          </div>
          <div className='flex justify-center items-center p-4'>
            <button className="btn btn-circle w-20 h-20 m-2 bg-base-100 hover:bg-base-200">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 stroke-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
            <button className="btn btn-circle m-2 bg-base-100 hover:bg-base-200">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" />
              </svg>
            </button>
            <button className="btn btn-circle w-20 h-20 m-2 bg-base-100 hover:bg-base-200">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-6 h-6 stroke-green-400">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
              </svg>
            </button>
          </div>
        </div>
        <div>
          test
        </div>
    </div>
  )
}

export default swipe
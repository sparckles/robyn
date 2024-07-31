import { motion } from 'framer-motion'

import { Footer } from '@/components/Footer'
import { BottomNavbar } from '@/components/documentation/BottomNavbar'
import { Navigation } from '@/components/documentation/Navigation'
import { Prose } from '@/components/documentation/Prose'
import { SectionProvider } from '@/components/documentation/SectionProvider'
import { useState, useEffect } from 'react'

export function Layout({ children, sections = [] }) {
  const [showSideBar, setShowSideBar] = useState(true)
  const [margin, setMargin] = useState('lg:ml-72 xl:ml-80')

  useEffect(() => {
    if (showSideBar) {
      setMargin('lg:ml-72 xl:ml-80')
    } else {
      setMargin('lg:ml-8 xl:ml-8')
    }
  }, [showSideBar])

  return (
    <SectionProvider sections={sections}>
      <div className={margin}>
        <motion.header
          layoutScroll
          className="contents lg:pointer-events-none lg:fixed lg:inset-0 lg:z-40 lg:flex"
        >
          <div className="contents lg:pointer-events-auto lg:block lg:w-72 lg:overflow-y-auto lg:border-white/10 lg:px-6 lg:pb-8 xl:w-80">
            <BottomNavbar />

            <HideIcon
              className="mt-8 h-12 w-12 fill-zinc-700"
              onClick={() => {
                setShowSideBar(!showSideBar)
              }}
            />

            {showSideBar && <Navigation className="hidden lg:mt-8 lg:block" />}
          </div>
        </motion.header>
        <div className="relative px-4 sm:px-6 lg:px-8">
          <main className="py-16">
            <Prose>{children}</Prose>
          </main>
          <Footer />
        </div>
      </div>
    </SectionProvider>
  )
}

function HideIcon(props) {
  return (
    <svg viewBox="0 -960 960 960" fill="#FFFFFF" {...props}>
      <path d="M560-240 320-480l240-240 56 56-184 184 184 184-56 56Z" />
    </svg>
  )
}

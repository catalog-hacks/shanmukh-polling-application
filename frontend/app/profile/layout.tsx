"use client";

import Link from "next/link";
import { useState, useRef, useEffect } from "react";
import {
  FaTachometerAlt,
  FaPoll,
  FaSignOutAlt,
  FaCaretDown,
  FaListAlt,
  FaChartBar,
  FaPlusCircle,
} from "react-icons/fa";
import clsx from "clsx";
import LogoutButton from "../_utils/LogoutButton";
import { useUser } from "../_utils/UserContext";

export default function ProfileLayout({ children }: { children: React.ReactNode }) {
  const [isDropdownOpen, setDropdownOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);
  const { user } = useUser();

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setDropdownOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  return (
    <div className="h-screen flex overflow-hidden" suppressHydrationWarning>
      <nav className="w-64 bg-gray-800 text-white flex-shrink-0">
        <div className="p-5 border-b border-gray-700">
          <h3 className="text-2xl font-semibold">Voting System</h3>
        </div>

        <ul className="flex flex-col mt-5 space-y-2 px-4">
          <li className="flex items-center gap-3 p-3 rounded-md hover:bg-gray-700">
            <FaTachometerAlt />
            <Link href="/profile">
              <span className="cursor-pointer">Dashboard</span>
            </Link>
          </li>

          <li className="flex items-center gap-3 p-3 rounded-md hover:bg-gray-700">
            <FaPlusCircle />
            <Link href="/profile/create-poll">
              <span className="cursor-pointer">Create Poll</span>
            </Link>
          </li>

          <li className="flex items-center gap-3 p-3 rounded-md hover:bg-gray-700">
            <FaListAlt />
            <Link href="/profile/all-polls">
              <span className="cursor-pointer">All Polls</span>
            </Link>
          </li>

          <li className="flex items-center gap-3 p-3 rounded-md hover:bg-gray-700">
            <FaPoll />
            <Link href="/profile/my-polls">
              <span className="cursor-pointer">My Polls</span>
            </Link>
          </li>

          <li className="flex items-center gap-3 p-3 rounded-md hover:bg-gray-700">
            <FaChartBar />
            <Link href="/profile/my-votes">
              <span className="cursor-pointer">My Votes</span>
            </Link>
          </li>
        </ul>
      </nav>

      <div className="flex-grow flex flex-col">
        <header className="bg-gray-700 text-white flex items-center justify-between px-6 py-4 shadow-md">
          <h2 className="text-lg font-semibold">Welcome to FairPolling</h2>
          <div className="relative">
            <span
              className="flex items-center gap-2 cursor-pointer"
              onClick={() => setDropdownOpen(!isDropdownOpen)}
            >
              {user?.fullName || "User"} <FaCaretDown />
            </span>

            <div
              ref={dropdownRef}
              className={clsx(
                "absolute right-0 mt-2 bg-gray-700 text-white rounded-md shadow-lg transition-opacity duration-300",
                {
                  "opacity-100 visible": isDropdownOpen,
                  "opacity-0 invisible": !isDropdownOpen,
                }
              )}
            >
              <div className="flex items-center gap-2 px-4 py-2 hover:bg-gray-800 cursor-pointer">
                <FaSignOutAlt />
                <LogoutButton />
              </div>
            </div>
          </div>
        </header>

        <main className="flex-grow overflow-y-auto p-6 bg-gray-100 scrollbar-hide">
          {children}
        </main>
      </div>
    </div>
  );
}

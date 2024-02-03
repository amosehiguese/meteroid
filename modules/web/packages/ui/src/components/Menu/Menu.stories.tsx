import { Mail as IconMail, Menu as IconMenu, RefreshCcw as IconRefreshCcw } from 'lucide-react'

import { ButtonAlt as Button } from '../ButtonAlt'
import { Divider } from '../Divider'

import { Menu } from './'

export default {
  title: 'Menu',
  component: Menu,
}

const SampleIcon = () => <IconMail size={14} strokeWidth={2} />

export const Default = () => (
  <Menu type="text">
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Divider />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)

export const withActiveState = () => (
  <Menu type="text">
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Divider />
    <Menu.Item active icon={<SampleIcon />}>
      Account settings
    </Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)

export const pills = () => (
  <Menu type="pills">
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Divider />
    <Menu.Item active icon={<SampleIcon />}>
      Account settings
    </Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)

export const border = () => (
  <Menu type="border">
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Divider />
    <Menu.Item active icon={<SampleIcon />}>
      Account settings
    </Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)

export const Groups = () => (
  <Menu type="text">
    <Menu.Group title="First group" />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Group title="Second group" />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)

export const PillsAndGroups = () => (
  <Menu type="pills">
    <Menu.Group title="First group" />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />} active>
      Account settings
    </Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Group title="Second group" />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)

export const BorderAndGroups = () => (
  <Menu type="border">
    <Menu.Group title="First group" />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />} active>
      Account settings
    </Menu.Item>
    <Menu.Group title="Second group" />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)

export const CustomNode = () => (
  <Menu type="border">
    <Menu.Group
      title={
        <>
          <div className="w-full flex items-center justify-between">
            <span>All tables</span>
            <button className="cursor-pointer">
              <IconRefreshCcw className="animate-spin" size={14} />
            </button>
          </div>
        </>
      }
    />
    <Menu.Item icon={<SampleIcon />}>
      <div className="flex items-center gap-2">
        <span>Account settings</span>
        <Button type="default">Button</Button>
      </div>
    </Menu.Item>
    <Menu.Item icon={<SampleIcon />}>
      <div className="flex items-center gap-2">
        <span>Account settings</span>
        <IconMenu size={14} strokeWidth={1.5} />
      </div>
    </Menu.Item>
    <Menu.Item icon={<SampleIcon />} active>
      Account settings
    </Menu.Item>
    <Menu.Group title="Second group" />
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
    <Menu.Item icon={<SampleIcon />}>Account settings</Menu.Item>
  </Menu>
)
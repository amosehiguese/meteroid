import { createConnectQueryKey, useMutation } from '@connectrpc/connect-query'
import { colors, radius, spaces } from '@md/foundation'
import { Flex, FormItem, Input, Modal, SidePanel } from '@md/ui'
import { styled } from '@stitches/react'
import { useQueryClient } from '@tanstack/react-query'
import { useState } from 'react'

import ConfirmationModal from '@/components/atoms/ConfirmationModal'
import { useZodForm } from '@/hooks/useZodForm'
import { schemas } from '@/lib/schemas'
import {
  createProduct,
  listProducts,
} from '@/rpc/api/products/v1/products-ProductsService_connectquery'

interface ProductEditPanelProps {
  visible: boolean
  closePanel: () => void
}
export const ProductEditPanel = ({ visible, closePanel }: ProductEditPanelProps) => {
  const [isClosingPanel, setIsClosingPanel] = useState(false)

  const queryClient = useQueryClient()

  const createProductMut = useMutation(createProduct, {
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: createConnectQueryKey(listProducts) })
    },
  })

  const methods = useZodForm({
    schema: schemas.products.createProductSchema,
    defaultValues: {
      name: '',
      description: '',
    },
  })
  const safeClosePanel = () => {
    const isDirty = methods.formState.isDirty
    if (isDirty) {
      setIsClosingPanel(true)
    } else {
      methods.reset()
      closePanel()
    }
  }

  // TODO try without the form, with onConfirm
  return (
    <>
      <SidePanel
        size="large"
        key="TableEditor"
        visible={visible}
        header={<SidePanel.HeaderTitle>Create a new product item</SidePanel.HeaderTitle>}
        className={`transition-all duration-100 ease-in `}
        onCancel={safeClosePanel}
        onConfirm={methods.handleSubmit(async values => {
          console.log('values', values)
          await createProductMut.mutateAsync(values)
          methods.reset()
          closePanel()
        })}
        onInteractOutside={event => {
          const isToast = (event.target as Element)?.closest('#toast')
          if (isToast) {
            event.preventDefault()
          }
        }}
      >
        <SidePanel.Content>
          <Flex direction="column" gap={spaces.space7}>
            <FormItem
              name="name"
              label="Product Name"
              error={methods.formState.errors.name?.message}
            >
              <Input type="text" placeholder="ACME Inc" {...methods.register('name')} />
            </FormItem>

            <FormItem
              name="description"
              label="Description"
              error={methods.formState.errors.description?.message}
            >
              <Input type="text" placeholder="desc" {...methods.register('description')} />
            </FormItem>
          </Flex>
        </SidePanel.Content>
      </SidePanel>
      <ConfirmationModal
        visible={isClosingPanel}
        header="Confirm to close"
        buttonLabel="Confirm"
        onSelectCancel={() => setIsClosingPanel(false)}
        onSelectConfirm={() => {
          setIsClosingPanel(false)
          methods.reset()
          closePanel()
        }}
      >
        <Modal.Content>
          <p className="py-4 text-sm text-scale-1100">
            There are unsaved changes. Are you sure you want to close the panel? Your changes will
            be lost.
          </p>
        </Modal.Content>
      </ConfirmationModal>
    </>
  )
}

const FixedCharges = styled('div', {
  padding: spaces.space3,
  backgroundColor: colors.neutral3,
  borderRadius: radius.radius3,
  display: 'flex',
  flexDirection: 'column',
  gap: spaces.space2,
})

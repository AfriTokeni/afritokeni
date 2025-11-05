<script lang="ts">
  import { Card, Button, Badge, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Modal, Tabs, TabItem, Avatar, Textarea } from 'flowbite-svelte';
  import { CheckCircle, XCircle, Eye } from '@lucide/svelte';
  
  let showReviewModal = $state(false);
  let selectedKYC = $state<any>(null);
  let rejectionReason = $state('');
  
  // Mock KYC data
  let pendingKYC = $state([
    {
      id: 'KYC-001',
      user: { name: 'John Doe', email: 'john@example.com', phone: '+234 801 234 5678', avatar: 'https://ui-avatars.com/api/?name=John+Doe' },
      type: 'user',
      documentType: 'National ID',
      documentNumber: 'NIN-12345678',
      submittedAt: '2024-11-05 10:30',
      documents: ['id_front.jpg', 'id_back.jpg', 'selfie.jpg'],
      status: 'pending'
    },
    {
      id: 'KYC-002',
      user: { name: 'Jane Smith', email: 'jane@example.com', phone: '+254 712 345 678', avatar: 'https://ui-avatars.com/api/?name=Jane+Smith' },
      type: 'agent',
      documentType: 'Passport',
      documentNumber: 'A12345678',
      location: 'Nairobi, Kenya',
      businessLicense: 'BL-987654',
      submittedAt: '2024-11-05 09:15',
      documents: ['passport.jpg', 'business_license.pdf', 'location_photo.jpg'],
      status: 'pending'
    },
    {
      id: 'KYC-003',
      user: { name: 'Bob Johnson', email: 'bob@example.com', phone: '+233 20 123 4567', avatar: 'https://ui-avatars.com/api/?name=Bob+Johnson' },
      type: 'user',
      documentType: 'Drivers License',
      documentNumber: 'DL-ABC123',
      submittedAt: '2024-11-05 08:45',
      documents: ['license_front.jpg', 'license_back.jpg'],
      status: 'pending'
    },
  ]);
  
  let approvedKYC = $state([
    {
      id: 'KYC-100',
      user: { name: 'Alice Brown', email: 'alice@example.com', phone: '+234 802 345 6789', avatar: 'https://ui-avatars.com/api/?name=Alice+Brown' },
      type: 'user',
      approvedAt: '2024-11-04 16:20',
      approvedBy: 'Admin User',
      status: 'approved'
    },
  ]);
  
  let rejectedKYC = $state([
    {
      id: 'KYC-200',
      user: { name: 'Charlie Wilson', email: 'charlie@example.com', phone: '+254 713 456 789', avatar: 'https://ui-avatars.com/api/?name=Charlie+Wilson' },
      type: 'agent',
      rejectedAt: '2024-11-04 14:10',
      rejectedBy: 'Admin User',
      reason: 'Document image quality too low',
      status: 'rejected'
    },
  ]);
  
  function reviewKYC(kyc: any) {
    selectedKYC = kyc;
    showReviewModal = true;
  }
  
  function approveKYC() {
    if (!selectedKYC) return;
    
    // Move from pending to approved
    pendingKYC = pendingKYC.filter(k => k.id !== selectedKYC.id);
    approvedKYC = [...approvedKYC, {
      ...selectedKYC,
      status: 'approved',
      approvedAt: new Date().toISOString(),
      approvedBy: 'Admin User'
    }];
    
    showReviewModal = false;
    selectedKYC = null;
  }
  
  function rejectKYC() {
    if (!selectedKYC || !rejectionReason) return;
    
    // Move from pending to rejected
    pendingKYC = pendingKYC.filter(k => k.id !== selectedKYC.id);
    rejectedKYC = [...rejectedKYC, {
      ...selectedKYC,
      status: 'rejected',
      rejectedAt: new Date().toISOString(),
      rejectedBy: 'Admin User',
      reason: rejectionReason
    }];
    
    showReviewModal = false;
    selectedKYC = null;
    rejectionReason = '';
  }
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">KYC Management</h1>
      <p class="mt-1 text-sm text-gray-500">Review and approve user verification requests</p>
    </div>
    <div class="flex gap-2">
      <Badge color="yellow" large>
        <span class="text-lg font-semibold">{pendingKYC.length}</span>
        <span class="ml-1">Pending</span>
      </Badge>
    </div>
  </div>
  
  <!-- KYC Tabs -->
  <Card>
    <Tabs>
      <TabItem open title="Pending ({pendingKYC.length})">
        <Table>
          <TableHead>
            <TableHeadCell>User</TableHeadCell>
            <TableHeadCell>Type</TableHeadCell>
            <TableHeadCell>Document</TableHeadCell>
            <TableHeadCell>Submitted</TableHeadCell>
            <TableHeadCell>Actions</TableHeadCell>
          </TableHead>
          <TableBody>
            {#each pendingKYC as kyc}
              <TableBodyRow>
                <TableBodyCell>
                  <div class="flex items-center gap-3">
                    <Avatar src={kyc.user.avatar} size="sm" />
                    <div>
                      <p class="font-medium text-gray-900">{kyc.user.name}</p>
                      <p class="text-sm text-gray-500">{kyc.user.email}</p>
                    </div>
                  </div>
                </TableBodyCell>
                <TableBodyCell>
                  <Badge color={kyc.type === 'agent' ? 'purple' : 'blue'}>
                    {kyc.type}
                  </Badge>
                </TableBodyCell>
                <TableBodyCell>
                  <p class="font-medium">{kyc.documentType}</p>
                  <p class="text-sm text-gray-500">{kyc.documentNumber}</p>
                </TableBodyCell>
                <TableBodyCell>{kyc.submittedAt}</TableBodyCell>
                <TableBodyCell>
                  <Button size="xs" onclick={() => reviewKYC(kyc)}>
                    <Eye class="mr-1 h-3 w-3" />
                    Review
                  </Button>
                </TableBodyCell>
              </TableBodyRow>
            {/each}
          </TableBody>
        </Table>
      </TabItem>
      
      <TabItem title="Approved ({approvedKYC.length})">
        <Table>
          <TableHead>
            <TableHeadCell>User</TableHeadCell>
            <TableHeadCell>Type</TableHeadCell>
            <TableHeadCell>Approved At</TableHeadCell>
            <TableHeadCell>Approved By</TableHeadCell>
          </TableHead>
          <TableBody>
            {#each approvedKYC as kyc}
              <TableBodyRow>
                <TableBodyCell>
                  <div class="flex items-center gap-3">
                    <Avatar src={kyc.user.avatar} size="sm" />
                    <div>
                      <p class="font-medium text-gray-900">{kyc.user.name}</p>
                      <p class="text-sm text-gray-500">{kyc.user.email}</p>
                    </div>
                  </div>
                </TableBodyCell>
                <TableBodyCell>
                  <Badge color={kyc.type === 'agent' ? 'purple' : 'blue'}>
                    {kyc.type}
                  </Badge>
                </TableBodyCell>
                <TableBodyCell>{kyc.approvedAt}</TableBodyCell>
                <TableBodyCell>{kyc.approvedBy}</TableBodyCell>
              </TableBodyRow>
            {/each}
          </TableBody>
        </Table>
      </TabItem>
      
      <TabItem title="Rejected ({rejectedKYC.length})">
        <Table>
          <TableHead>
            <TableHeadCell>User</TableHeadCell>
            <TableHeadCell>Type</TableHeadCell>
            <TableHeadCell>Rejected At</TableHeadCell>
            <TableHeadCell>Reason</TableHeadCell>
          </TableHead>
          <TableBody>
            {#each rejectedKYC as kyc}
              <TableBodyRow>
                <TableBodyCell>
                  <div class="flex items-center gap-3">
                    <Avatar src={kyc.user.avatar} size="sm" />
                    <div>
                      <p class="font-medium text-gray-900">{kyc.user.name}</p>
                      <p class="text-sm text-gray-500">{kyc.user.email}</p>
                    </div>
                  </div>
                </TableBodyCell>
                <TableBodyCell>
                  <Badge color={kyc.type === 'agent' ? 'purple' : 'blue'}>
                    {kyc.type}
                  </Badge>
                </TableBodyCell>
                <TableBodyCell>{kyc.rejectedAt}</TableBodyCell>
                <TableBodyCell>
                  <p class="text-sm text-gray-600">{kyc.reason}</p>
                </TableBodyCell>
              </TableBodyRow>
            {/each}
          </TableBody>
        </Table>
      </TabItem>
    </Tabs>
  </Card>
</div>

<!-- KYC Review Modal -->
<Modal bind:open={showReviewModal} size="xl" title="Review KYC Submission">
  {#if selectedKYC}
    <div class="space-y-6">
      <!-- User Info -->
      <div class="flex items-center gap-4 rounded-lg border border-gray-200 p-4">
        <Avatar src={selectedKYC.user.avatar} size="lg" />
        <div>
          <h3 class="text-lg font-semibold text-gray-900">{selectedKYC.user.name}</h3>
          <p class="text-sm text-gray-500">{selectedKYC.user.email}</p>
          <p class="text-sm text-gray-500">{selectedKYC.user.phone}</p>
          <Badge color={selectedKYC.type === 'agent' ? 'purple' : 'blue'} class="mt-2">
            {selectedKYC.type}
          </Badge>
        </div>
      </div>
      
      <!-- Document Details -->
      <div class="space-y-3">
        <h4 class="font-semibold text-gray-900">Document Information</h4>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <p class="text-sm text-gray-500">Document Type</p>
            <p class="font-medium">{selectedKYC.documentType}</p>
          </div>
          <div>
            <p class="text-sm text-gray-500">Document Number</p>
            <p class="font-medium">{selectedKYC.documentNumber}</p>
          </div>
          {#if selectedKYC.location}
            <div>
              <p class="text-sm text-gray-500">Location</p>
              <p class="font-medium">{selectedKYC.location}</p>
            </div>
          {/if}
          {#if selectedKYC.businessLicense}
            <div>
              <p class="text-sm text-gray-500">Business License</p>
              <p class="font-medium">{selectedKYC.businessLicense}</p>
            </div>
          {/if}
        </div>
      </div>
      
      <!-- Documents Preview -->
      <div class="space-y-3">
        <h4 class="font-semibold text-gray-900">Uploaded Documents</h4>
        <div class="grid grid-cols-3 gap-4">
          {#each selectedKYC.documents as doc}
            <div class="rounded-lg border border-gray-200 p-4 text-center">
              <div class="mb-2 flex h-32 items-center justify-center bg-gray-100">
                <p class="text-sm text-gray-500">📄 {doc}</p>
              </div>
              <Button size="xs" color="light">View Full</Button>
            </div>
          {/each}
        </div>
      </div>
      
      <!-- Rejection Reason (if rejecting) -->
      <div class="space-y-2">
        <label for="rejection-reason" class="text-sm font-medium text-gray-900">
          Rejection Reason (optional)
        </label>
        <Textarea
          id="rejection-reason"
          bind:value={rejectionReason}
          placeholder="Provide a reason if rejecting this KYC..."
          rows={3}
        />
      </div>
    </div>
    
    {#snippet footer()}
      <div class="flex gap-2">
        <Button color="green" onclick={approveKYC}>
          <CheckCircle class="mr-2 h-4 w-4" />
          Approve KYC
        </Button>
        <Button color="red" onclick={rejectKYC} disabled={!rejectionReason}>
          <XCircle class="mr-2 h-4 w-4" />
          Reject KYC
        </Button>
        <Button color="light" onclick={() => { showReviewModal = false; rejectionReason = ''; }}>
          Cancel
        </Button>
      </div>
    {/snippet}
  {/if}
</Modal>

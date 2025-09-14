import CardModal from "./card/modal";
import CardTrashedModal from "./card/trashed/modal";
import MerchantModal from "./merchant/modal";
import MerchantTrashedModal from "./merchant/trashed/modal";
import SaldoModal from "./saldo/modal";
import SaldoTrashedModal from "./saldo/trashed/modal";
import TopupModal from "./topup/modal";
import TopupTrashedModal from "./topup/trashed/modal";
import TransactionModal from "./transaction/modal";
import TransactionTrashedModal from "./transaction/trashed/modal";
import TransferModal from "./transfer/modal";
import TransferTrashedModal from "./transfer/trashed/modal";
import UserModal from "./user/modal";
import UserTrashedModal from "./user/trashed/modal";
import RoleModal from "./role/modal";
import RoleTrashedModal from "./role/trashed/modal";
import WithdrawModal from "./withdraw/modal";
import WithdrawTrashedModal from "./withdraw/trashed/modal";

export default function ModalAdmin() {
  return (
    <>
      <UserModal />
      <UserTrashedModal />
      <RoleModal />
      <RoleTrashedModal />
      <CardModal />
      <CardTrashedModal />
      <MerchantModal />
      <MerchantTrashedModal />
      <SaldoModal />
      <SaldoTrashedModal />
      <TopupModal />
      <TopupTrashedModal />
      <TransactionModal />
      <TransactionTrashedModal />
      <TransferModal />
      <TransferTrashedModal />
      <WithdrawModal />
      <WithdrawTrashedModal />
    </>
  );
}

import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  CreateTransactionFormValues,
  createTransactionRequestSchema,
} from "@/schemas";
import { zodResolver } from "@hookform/resolvers/zod";
import { useToast } from "@/hooks/use-toast";
import useCardStore from "@/store/card/card";
import { debounce } from "@/helpers/debounce";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import AsyncSelect from "react-select/async";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import { TransactionCreateFormProps } from "@/types/form";
import useMerchantStore from "@/store/merchant/merchant";
import { FindAllCard, FindAllMerchant } from "@/types/domain/request";

interface CardOption {
  value: string;
  label: string;
}

interface MerchantOption {
  value: number;
  label: string;
}

const paymentMethodOptions = [
  { value: "visa", label: "Visa" },
  { value: "mastercard", label: "Mastercard" },
  { value: "discover", label: "Discover" },
  { value: "american_express", label: "American Express" },
  { value: "bri", label: "BRI" },
  { value: "mandiri", label: "Mandiri" },
  { value: "bca", label: "BCA" },
  { value: "bni", label: "BNI" },
  { value: "bukopin", label: "Bukopin" },
  { value: "e_banking", label: "E-Banking" },
  { value: "dana", label: "DANA" },
  { value: "ovo", label: "OVO" },
  { value: "gopay", label: "GoPay" },
  { value: "linkaja", label: "LinkAja" },
  { value: "jenius", label: "Jenius" },
  { value: "fastpay", label: "FastPay" },
  { value: "kudo", label: "Kudo" },
  { value: "paypal", label: "PayPal" },
  { value: "alfamart", label: "Alfamart" },
  { value: "indomart", label: "Indomart" },
  { value: "lawson", label: "Lawson" },
];

const CreateTransactionForm = forwardRef<
  HTMLFormElement,
  TransactionCreateFormProps
>(({ onSubmit }, ref) => {
  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
  } = useForm<CreateTransactionFormValues>({
    resolver: zodResolver(createTransactionRequestSchema),
    defaultValues: {
      card_number: "",
      amount: 0,
      payment_method: "credit_card",
      merchant_id: 0,
      transaction_time: new Date(),
    },
  });

  const { toast } = useToast();

  const { cards, loadingGetCards, setLoadingGetCards, findAllCards } =
    useCardStore();

  const {
    merchants,
    loadingGetMerchants,
    setLoadingGetMerchants,
    findAllMerchants,
  } = useMerchantStore();

  const [cardOptions, setCardOptions] = useState<CardOption[]>([]);
  const [merchantOptions, setMerchantOptions] = useState<MerchantOption[]>([]);

  useEffect(() => {
    const fetchInitialCards = async () => {
      setLoadingGetCards(true);
      try {
        const searchReq: FindAllCard = {
          search: "",
          page: 1,
          page_size: 10,
          toast: toast,
        };
        await findAllCards(searchReq);

        if (cards) {
          const options = cards.map((card) => ({
            value: card.card_number,
            label: card.card_number,
          }));

          console.log("card", cards);
          setCardOptions(options);
        }
      } catch (error) {
        console.error("Failed to fetch initial cards:", error);
      } finally {
        setLoadingGetCards(false);
      }
    };

    fetchInitialCards();
  }, []);

  useEffect(() => {
    const fetchInitialMerchants = async () => {
      setLoadingGetMerchants(true);
      try {
        const searchReq: FindAllMerchant = {
          search: "",
          page: 1,
          page_size: 10,
          toast: toast,
        };
        await findAllMerchants(searchReq);

        if (merchants) {
          const options = merchants.map((merchant) => ({
            value: merchant.id,
            label: merchant.name,
          }));

          console.log("merchant", merchants);
          setMerchantOptions(options);
        }
      } catch (error) {
        console.error("Failed to fetch initial merchants:", error);
      } finally {
        setLoadingGetMerchants(false);
      }
    };

    fetchInitialMerchants();
  }, []);

  const loadOptionCards = useCallback(
    debounce(
      async (inputValue: string, callback: (options: CardOption[]) => void) => {
        setLoadingGetCards(true);
        try {
          const searchReq: FindAllCard = {
            search: inputValue,
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllCards(searchReq);

          if (cards) {
            const filteredOptions = cards
              .filter((card) =>
                card.card_number
                  .toLowerCase()
                  .includes(inputValue.toLowerCase()),
              )
              .map((card) => ({
                value: card.card_number,
                label: card.card_number,
              }));

            callback(filteredOptions);
          }
        } catch (error) {
          console.error("Failed to fetch cards:", error);
        } finally {
          setLoadingGetCards(false);
        }
      },
      500,
    ),
    [cards, findAllCards, toast],
  );

  const loadOptionMerchants = useCallback(
    debounce(
      async (
        inputValue: string,
        callback: (options: MerchantOption[]) => void,
      ) => {
        setLoadingGetMerchants(true);
        try {
          const searchReq: FindAllMerchant = {
            search: inputValue,
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllMerchants(searchReq);

          if (merchants) {
            const filteredOptions = merchants
              .filter((merchant) =>
                merchant.name.toLowerCase().includes(inputValue.toLowerCase()),
              )
              .map((merchant) => ({
                value: merchant.id,
                label: merchant.name,
              }));

            callback(filteredOptions);
          }
        } catch (error) {
          console.error("Failed to fetch merchants:", error);
        } finally {
          setLoadingGetMerchants(false);
        }
      },
      500,
    ),
    [merchants, findAllMerchants, toast],
  );

  return (
    <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div>
        <Label htmlFor="cardNumber" className="text-sm font-medium">
          Card Number
        </Label>
        <Controller
          name="card_number"
          control={control}
          render={({ field }) => (
            <AsyncSelect
              {...field}
              loadOptions={loadOptionCards}
              defaultOptions={cardOptions}
              isLoading={loadingGetCards}
              placeholder="Select a card"
              className="mt-1"
              styles={{
                control: (base) => ({
                  ...base,
                  borderColor: errors.card_number
                    ? "#ef4444"
                    : base.borderColor,
                  borderRadius: "0.375rem",
                  padding: "0.25rem",
                  boxShadow: "none",
                  "&:hover": {
                    borderColor: "#3b82f6",
                  },
                }),
                option: (base, { isFocused, isSelected }) => ({
                  ...base,
                  backgroundColor: isSelected
                    ? "#3b82f6"
                    : isFocused
                      ? "#bfdbfe"
                      : "white",
                  color: isSelected ? "white" : "black",
                  "&:hover": {
                    backgroundColor: "#bfdbfe",
                  },
                }),
                menu: (base) => ({
                  ...base,
                  borderRadius: "0.375rem",
                  boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                }),
              }}
              value={cardOptions.find((option) => option.value === field.value)}
              onChange={(selectedOption) => {
                field.onChange(selectedOption ? selectedOption.value : null);
              }}
            />
          )}
        />
        {errors.card_number && (
          <p className="text-red-500 text-sm mt-1">
            {errors.card_number.message}
          </p>
        )}
      </div>
      <div>
        <Label htmlFor="cardNumber" className="text-sm font-medium">
          Merchant ID
        </Label>
        <Controller
          name="merchant_id"
          control={control}
          render={({ field }) => (
            <AsyncSelect
              {...field}
              loadOptions={loadOptionMerchants}
              defaultOptions={merchantOptions}
              isLoading={loadingGetMerchants}
              placeholder="Select a merchant"
              className="mt-1"
              styles={{
                control: (base) => ({
                  ...base,
                  borderColor: errors.merchant_id
                    ? "#ef4444"
                    : base.borderColor,
                  borderRadius: "0.375rem",
                  padding: "0.25rem",
                  boxShadow: "none",
                  "&:hover": {
                    borderColor: "#3b82f6",
                  },
                }),
                option: (base, { isFocused, isSelected }) => ({
                  ...base,
                  backgroundColor: isSelected
                    ? "#3b82f6"
                    : isFocused
                      ? "#bfdbfe"
                      : "white",
                  color: isSelected ? "white" : "black",
                  "&:hover": {
                    backgroundColor: "#bfdbfe",
                  },
                }),
                menu: (base) => ({
                  ...base,
                  borderRadius: "0.375rem",
                  boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                }),
              }}
              value={merchantOptions.find(
                (option) => option.value === field.value,
              )}
              onChange={(selectedOption) => {
                field.onChange(selectedOption ? selectedOption.value : null);
              }}
            />
          )}
        />
        {errors.merchant_id && (
          <p className="text-red-500 text-sm mt-1">
            {errors.merchant_id.message}
          </p>
        )}
      </div>
      <div>
        <Label htmlFor="amount" className="text-sm font-medium">
          Amount
        </Label>
        <Input
          id="amount"
          type="number"
          placeholder="Enter amount"
          className="mt-1"
          {...register("amount", { valueAsNumber: true })}
        />
        {errors.amount && (
          <p className="text-red-500 text-sm mt-1">{errors.amount.message}</p>
        )}
      </div>
      <div>
        <Label htmlFor="payment_method" className="text-sm font-medium">
          Payment Method
        </Label>
        <Controller
          name="payment_method"
          control={control}
          render={({ field }) => (
            <Select onValueChange={field.onChange} value={field.value}>
              <SelectTrigger className="w-full mt-1">
                <SelectValue placeholder="Select payment method" />
              </SelectTrigger>
              <SelectContent>
                {paymentMethodOptions.map((option) => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          )}
        />
        {errors.payment_method && (
          <p className="text-red-500 text-sm mt-1">
            {errors.payment_method.message}
          </p>
        )}
      </div>

      <div>
        <Label htmlFor="transaction_time" className="text-sm font-medium">
          Transaction Time
        </Label>
        <Controller
          name="transaction_time"
          control={control}
          render={({ field }) => {
            const value = new Date(field.value).toISOString().slice(0, 16);

            return (
              <Input
                id="transaction_time"
                type="datetime-local"
                className="mt-1"
                value={value}
                onChange={(e) => {
                  field.onChange(new Date(e.target.value));
                }}
              />
            );
          }}
        />
        {errors.transaction_time && (
          <p className="text-red-500 text-sm mt-1">
            {errors.transaction_time.message}
          </p>
        )}
      </div>
    </form>
  );
});

CreateTransactionForm.displayName = "CreateTransactionForm";

export default CreateTransactionForm;

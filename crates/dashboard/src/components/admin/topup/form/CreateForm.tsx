import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { CreateTopupFormValues, createTopupRequestSchema } from "@/schemas";
import { useToast } from "@/hooks/use-toast";
import useCardStore from "@/store/card/card";
import { debounce } from "@/helpers/debounce";
import { TopupCreateFormProps } from "@/types/form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Select as ShadcnSelect,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import AsyncSelect from "react-select/async";
import { FindAllCard } from "@/types/domain/request";

const paymentProviders = [
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

interface CardOption {
  value: string;
  label: string;
}

const CreateTopupForm = forwardRef<HTMLFormElement, TopupCreateFormProps>(
  ({ onSubmit }, ref) => {
    const {
      register,
      control,
      handleSubmit,
      formState: { errors },
    } = useForm<CreateTopupFormValues>({
      resolver: zodResolver(createTopupRequestSchema),
      defaultValues: {
        card_number: "",
        topup_amount: 0,
        topup_method: "credit_card",
      },
    });

    const { toast } = useToast();

    const { cards, loadingGetCards, setLoadingGetCards, findAllCards } =
      useCardStore();

    const [cardOptions, setCardOptions] = useState<CardOption[]>([]);

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

    const loadOptions = useCallback(
      debounce(
        async (
          inputValue: string,
          callback: (options: CardOption[]) => void,
        ) => {
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
                loadOptions={loadOptions}
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
                value={cardOptions.find(
                  (option) => option.value === field.value,
                )}
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
          <Label htmlFor="topup_amount" className="text-sm font-medium">
            Top-up Amount
          </Label>
          <Input
            id="topup_amount"
            type="number"
            placeholder="Enter top-up amount"
            className="mt-1"
            {...register("topup_amount", { valueAsNumber: true })}
          />
          {errors.topup_amount && (
            <p className="text-red-500 text-sm mt-1">
              {errors.topup_amount.message}
            </p>
          )}
        </div>
        <div>
          <Label htmlFor="cardType" className="text-sm font-medium">
            Topup Method
          </Label>
          <Controller
            name="topup_method"
            control={control}
            render={({ field }) => (
              <ShadcnSelect onValueChange={field.onChange} value={field.value}>
                <SelectTrigger className="w-full mt-1">
                  <SelectValue placeholder="Select card type" />
                </SelectTrigger>
                <SelectContent>
                  {paymentProviders.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </ShadcnSelect>
            )}
          />
          {errors.topup_method && (
            <p className="text-red-500 text-sm mt-1">
              {errors.topup_method.message}
            </p>
          )}
        </div>
      </form>
    );
  },
);

CreateTopupForm.displayName = "CreateTopupForm";

export default CreateTopupForm;

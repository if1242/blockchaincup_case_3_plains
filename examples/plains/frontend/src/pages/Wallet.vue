<template>
  <div>
    <navbar/>

    <div class="container">
      <div class="row">
        <div class="col-md-6">
          <div class="card mt-5">
            <div class="card-header">Информация о пользователе</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Имя:</strong></div>
                  <div class="col-sm-9">{{ name }}</div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Public key:</strong></div>
                  <div class="col-sm-9"><code>{{ keyPair.publicKey }}</code></div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Детали в наличии:</strong></div>
                  <div class="col-sm-9">
                  
                  
                  
                    <div v-for="detail in details">
                    <div v-for="variant in variantsDetails">
                    <span v-if="variant.name_of_type === detail.name_of_type">{{ variant.id }} : {{detail.count}}</span>
                                                                                  
                    </div>
                     </div>                    
                          
                  </div>
                </div>
              </li>
              <li class="list-group-item">
                <div class="row">
                  <div class="col-sm-3"><strong>Производит:</strong></div>
                  <div class="col-sm-9">
                  
                    <div v-for="candevelop in develop">
                    <div v-for="variant in variantsDetails">
                    <span v-if="variant.name_of_type === candevelop.name_of_type">{{ variant.id }} : {{candevelop.count}}</span>
                                                                                  
                    </div>
                     </div> 
                                         
                          
                  </div>
                </div>
              </li>
            </ul>
          </div>

          <div class="card mt-5">
            <div class="card-header">Транзакции</div>
            <ul class="list-group list-group-flush">
              <li class="list-group-item font-weight-bold">
                <div class="row">
                  <div class="col-sm-12">Описание</div>
                </div>
              </li>
              <!-- eslint-disable-next-line vue/require-v-for-key -->
              <li v-for="transaction in reverseTransactions" class="list-group-item">
                <div class="row">
                  <div class="col-sm-12">
                    <router-link :to="{ name: 'transaction', params: { hash: transaction.hash } }">
                      <span v-if="transaction.message_id === 4">Wallet created</span>
                      <span v-else-if="transaction.message_id === 2">
                        <strong v-numeral="transaction.body.amount"/> funds added
                      </span>
                      <span v-else-if="transaction.message_id === 0 && transaction.body.from === keyPair.publicKey">
                        <strong v-numeral="transaction.body.amount"/> funds sent
                      </span>
                      <span v-else-if="transaction.message_id === 0 && transaction.body.to === keyPair.publicKey">
                        <strong v-numeral="transaction.body.amount"/> funds received
                      </span>
                      <span v-else-if="transaction.message_id === 1 && transaction.body.from === keyPair.publicKey">
                        <strong v-numeral="transaction.body.count"/> details send
                      </span>
                      <span v-else-if="transaction.message_id === 1 && transaction.body.to === keyPair.publicKey">
                        <strong v-numeral="transaction.body.count"/> details received
                      </span>
                      <span v-else-if="transaction.message_id === 3">
                        <strong v-numeral="transaction.body.name_of_type"/> details created
                      </span>
                    </router-link>
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>
    
        
        
        <div class="col-md-6">
          <div class="card mt-5">
            <div class="card-header">Произвести деталь</div>
            <div class="card-body">
              <form @submit.prevent="addDetails">
                <div class="form-group">
                  <label class="d-block">Что производим?</label>
                   <label>Двигатель = Турбина + Компрессор</label></br>
                   <label>Шасси = Колесо + 5 болтов</label></br>
        <label>Корпус = 2 крыла + 10 порций клея + 1 фюзеляж</label></br>
        <label>Самолет = 2 Двигателя + 3 Шасси + Корпус</label></br>
       
                  <!-- eslint-disable-next-line vue/require-v-for-key -->
                  <div v-for="variant in variantsDetails" class="form-check form-check-inline">
                    <input :id="variant.id" :value="variant.name_of_type" :checked="name_of_typeToAdd == variant.name_of_type" v-model="name_of_typeToAdd" class="form-check-input" type="radio"></br>
                    <label :for="variant.id" class="form-check-label">{{ variant.id }}</label>
                  </div>
                </div>
                <button type="submit" class="btn btn-primary">Произвести</button>
              </form>
            </div>
          </div>

          <div class="card mt-5">
            <div class="card-header">Передать деталь</div>
            <div class="card-body">
              
              
              <form @submit.prevent="transferDetails">
                <div class="form-group">
                  <label>Кому:</label>
                  <input v-model="receiver" type="text" class="form-control" placeholder="Введите public key" required>
                 </div>
                <div class="form-group">
                  <label>Сколько?:</label>
                  <div class="input-group">
                    
                    <input v-model="countToTransfer" type="number" class="form-control" placeholder="Введите количество" min="0" required>
                  </div>
                  <label>Тип детали:</label>
                  <div class="input-group">
                    
                    <select v-model="name_of_typeToTransfer">
                        <option v-for="variant in variantsDetails" v-bind:value="variant.name_of_type">
                        {{ variant.id }}
                        </option>
                    </select>
                    
                  </div>
                </div>
                <button type="submit" class="btn btn-primary">Передать</button>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>

    <spinner :visible="isSpinnerVisible"/>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import Modal from '../components/Modal.vue'
  import Navbar from '../components/Navbar.vue'
  import Spinner from '../components/Spinner.vue'

  module.exports = {
    components: {
      Modal,
      Navbar,
      Spinner
    },
    data() {
      return {
        name: '',
        balance: 0,
        amountToAdd: 10,
        name_of_typeToAdd: 7,
        details: [{name_of_type: null, count: null}],
        develop: [{name_of_type: null, count: null}],
        receiver: '',
        amountToTransfer: '',
        countToTransfer: '',
        name_of_typeToTransfer: '',
        isSpinnerVisible: false,
        transactions: [],
        variants: [
          { id: 'ten', amount: 10 },
          { id: 'fifty', amount: 50 },
          { id: 'hundred', amount: 100 }
        ],
        variantsDetails: [
          { id: 'Самолет',  name_of_type: 0 },
          { id: 'Двигатель',  name_of_type: 1 },
          { id: 'Шасси',  name_of_type: 2 },
          { id: 'Корпус',  name_of_type: 3 },
          { id: 'Турбина',  name_of_type: 4 },
          { id: 'Компрессор',  name_of_type: 5 },
          { id: 'Колесо',  name_of_type: 6 },
          { id: 'Болт', name_of_type: 7 },
          { id: 'Крыло', name_of_type: 8 },
          { id: 'Клей', name_of_type: 9 },
          { id: 'Фюзеляж', name_of_type: 10 }
        ] 
      }
    },
    computed: Object.assign({
      reverseTransactions() {
        return this.transactions.slice().reverse()
      }
    }, mapState({
      keyPair: state => state.keyPair
    })),
    methods: {
      async loadUser() {
        if (this.keyPair === null) {
          this.$store.commit('logout')
          this.$router.push({ name: 'home' })
          return
        }

        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.getWallet(this.keyPair.publicKey)
          this.name = data.wallet.name
          this.balance = data.wallet.balance
          this.details = data.wallet.details
          this.develop = data.wallet.develop
          this.transactions = data.transactions
          this.isSpinnerVisible = false
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async addFunds() {
        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.addFunds(this.keyPair, this.amountToAdd)
          this.balance = data.wallet.balance
          this.details = data.wallet.details
          this.develop = data.wallet.develop
          this.transactions = data.transactions
          this.isSpinnerVisible = false
          this.$notify('success', 'Add funds transaction has been written into the blockchain')
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },
      
      async addDetails() {
        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.addDetails(this.keyPair, this.name_of_typeToAdd)
          this.balance = data.wallet.balance
          this.details = data.wallet.details
          this.develop = data.wallet.develop
          this.transactions = data.transactions
          this.isSpinnerVisible = false
          this.$notify('success', 'Add funds transaction has been written into the blockchain')
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },

      async transfer() {
        if (!this.$validateHex(this.receiver)) {
          return this.$notify('error', 'Invalid public key is passed')
        }

        if (this.receiver === this.keyPair.publicKey) {
          return this.$notify('error', 'Can not transfer funds to yourself')
        }

        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.transfer(this.keyPair, this.receiver, this.amountToTransfer)
          this.balance = data.wallet.balance
          this.details = data.wallet.details
          this.develop = data.wallet.develop
          this.transactions = data.transactions
          this.isSpinnerVisible = false
          this.$notify('success', 'Transfer transaction has been written into the blockchain')
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      },
      
      async transferDetails() {
        if (!this.$validateHex(this.receiver)) {
          return this.$notify('error', 'Invalid public key is passed')
        }

        if (this.receiver === this.keyPair.publicKey) {
          return this.$notify('error', 'Can not transfer funds to yourself')
        }

        this.isSpinnerVisible = true

        try {
          const data = await this.$blockchain.transferDetails(this.keyPair, this.receiver, this.name_of_typeToTransfer, this.countToTransfer)
          this.balance = data.wallet.balance
          this.details = data.wallet.details
          this.develop = data.wallet.develop
          this.transactions = data.transactions
          this.isSpinnerVisible = false
          this.$notify('success', 'Transfer transaction has been written into the blockchain')
        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }
      }
      
    },
    mounted() {
      this.$nextTick(function() {
        this.loadUser()
      })
    }
  }
</script>

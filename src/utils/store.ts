import { SignState } from '../models/SignState';
import { makeAutoObservable } from 'mobx';

export default class Store {
  auth: boolean = false;
  signState: SignState = SignState.None;

  setAuth(value: boolean) {
    this.auth = value;
  }

  setSignState(value: SignState) {
    this.signState = value;
  }

  constructor() {
    makeAutoObservable(this);
  }
}
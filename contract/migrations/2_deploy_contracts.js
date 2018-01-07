var Migrations = artifacts.require('./Migrations.sol');
var Storage = artifacts.require('./Storage.sol');
var YadqlStorage = artifacts.require('./YadqlStorage.sol');

module.exports = function(deployer) {
  deployer.deploy(Storage).then(function() {
    return deployer.deploy(YadqlStorage, Storage.address);
  });
}
